package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	if head == nil {
		return false
	}

	visited := make(map[*ListNode]bool)
	curr := head
	for curr.Next != nil {
		if visited[curr] {
			// already visited a node with this same pointer - cycle
			return true
		}
		visited[curr] = true
		curr = curr.Next
	}
	return false
}

// solve w/ O(1) space complexity if you use a fast pointer (move by two each time) and a slow pointer

func main() {
}
