package main

import (
	"container/heap"
	"fmt"
)

// An Item is something we manage in a priority queue.
type Item struct {
	value    string // The value of the item; arbitrary.
	priority int    // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // don't stop the GC from reclaiming the item eventually
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// update modifies the priority and value of an Item in the queue.
func (pq *PriorityQueue) update(item *Item, value string, priority int) {
	item.value = value
	item.priority = priority
	heap.Fix(pq, item.index)
}

func smallestDiverse(a int, b int, c int) string {
	pq := make(PriorityQueue, 3)
	pq[0] = &Item{
		value:    "a",
		priority: a,
	}
	pq[1] = &Item{
		value:    "b",
		priority: b,
	}
	pq[2] = &Item{
		value:    "c",
		priority: c,
	}
	heap.Init(&pq)

	res := ""
	for pq.Len() != 0 {
		item := heap.Pop(&pq).(*Item)
		if item.priority == 0 {
			continue
		}

		res += item.value
		// add item again with reduced priority
		newItem := &Item{
			value:    item.value,
			priority: item.priority - 1,
		}
		if newItem.priority > 0 {
			heap.Push(&pq, newItem)
		}
	}
	return res
}

func main() {
	fmt.Println(smallestDiverse(3, 1, 0))
	fmt.Println(smallestDiverse(1, 4, 4))
	fmt.Println(smallestDiverse(1, 3, 0))
}
