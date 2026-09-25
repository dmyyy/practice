package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	// count list size
	nodeCount := 0
	curr := head
	for curr != nil {
		curr = curr.Next
		nodeCount += 1
	}

	// find nth node
	var prev *ListNode
	curr = head
	for nodeCount != n {
		prev = curr
		curr = curr.Next
		nodeCount -= 1
	}

	// remove nth node
	if prev == nil {
		// remove first node
		head = head.Next
	} else {
		prev.Next = curr.Next
	}

	return head
}

func main() {
}
