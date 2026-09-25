package main

type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	if head == nil {
		return nil
	}

	// mapping from nodes we've seen while iterating to nodes in new list
	nodesToNewNodes := make(map[*Node]*Node)
	nodesToNewNodes[nil] = nil

	newHead := &Node{}
	nodesToNewNodes[head] = newHead

	curr := head
	newCurr := newHead
	for curr != nil {
		newCurr.Val = curr.Val

		newNode, ok := nodesToNewNodes[curr.Random]
		if !ok {
			newNode = &Node{}
			nodesToNewNodes[curr.Random] = newNode
		}
		newCurr.Random = newNode

		if newNext, ok := nodesToNewNodes[curr.Next]; ok {
			newCurr.Next = newNext
		} else {
			newCurr.Next = &Node{}
			nodesToNewNodes[curr.Next] = newCurr.Next
		}

		curr = curr.Next
		newCurr = newCurr.Next
	}

	return newHead
}

func main() {
}
