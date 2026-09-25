package main

type LRUCache struct {
	nodes    map[int]*Node
	head     *Node
	tail     *Node
	capacity int
}

type Node struct {
	key  int
	val  int
	next *Node
	prev *Node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		make(map[int]*Node, capacity),
		nil,
		nil,
		capacity,
	}
}

func (this *LRUCache) Get(key int) int {
	var node *Node
	var ok bool
	if node, ok = this.nodes[key]; !ok {
		return -1
	}

	if node != this.head {
		// fix previous
		if node.prev != nil {
			node.prev.next = node.next
		}
		// fix next
		if node.next != nil {
			node.next.prev = node.prev
		}
		// move node to head
		node.prev = nil
		node.next = this.head
		this.head.prev = node
		this.head = node
	}

	return node.val
}

// not handling duplicates correctly?
// not setting the tail correctly on adding the second element...

func (this *LRUCache) Put(key int, value int) {
	if len(this.nodes) == this.capacity {
		// evict least recently used element to make space
		lru := this.tail
		if lru != nil {
			delete(this.nodes, lru.key)
			if lru.prev != nil {
				lru.prev.next = nil
			}
			this.tail = lru.prev
			lru.prev = nil
		}
	}

	node := &Node{
		key,
		value,
		this.head,
		nil,
	}
	if this.head != nil {
		this.head.prev = node
	}
	this.head = node
	if this.tail == nil {
		this.tail = node.next
	}
	this.nodes[key] = node
}
