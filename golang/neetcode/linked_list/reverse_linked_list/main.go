package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	curr := head
	next := head.Next
	curr.Next = nil
	for next.Next != nil {
		tmp := next
		next = next.Next
		tmp.Next = curr
		curr = tmp
	}
	next.Next = curr
	return next
}

func main() {
}
