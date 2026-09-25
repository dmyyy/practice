package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func reorderList(head *ListNode) {
	if head == nil || head.Next == nil || head.Next.Next == nil {
		// already in order for lists of size 0,1,2
		return
	}

	// stop slow half-way through the linked list
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow, fast = slow.Next, fast.Next.Next
	}

	// reverse direction of links from slow till end of linked list
	//
	// IMPORTANT: need to break the link on the back edge here
	var prev *ListNode
	second := slow.Next
	slow.Next = nil
	for second != nil {
		tmp := second
		second = second.Next
		tmp.Next = prev
		prev = tmp
	}

	// interleave linked list
	curr := head
	end := prev
	for end != nil {
		tmp1, tmp2 := curr.Next, end.Next
		curr.Next = end
		end.Next = tmp1
		curr, end = tmp1, tmp2
	}
}

func main() {
}
