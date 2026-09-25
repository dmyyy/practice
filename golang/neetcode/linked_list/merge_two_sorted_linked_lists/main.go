package main

type ListNode struct {
	Val  int
	Next *ListNode
}

// had issues handling all edge cases the way I was doing it...
//

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}

	mergeList := list1
	otherList := list2
	if list1.Val > list2.Val {
		mergeList = list2
		otherList = list1
	}

	curr := mergeList
	for curr != nil {
		if otherList == nil {
			// finished merging
			break
		}

		for curr.Next != nil && curr.Next.Val < otherList.Val {
			// keep going in merged list as long as val is less than the one in other list
			curr = curr.Next
		}
		if curr.Val < otherList.Val {
			// all vals in curr are smaller than head of other list and we hit the end of the list
			break
		}

		// next val is larger - merge head of otherList into mergeList
		next := curr.Next
		curr.Next = otherList
		otherList = otherList.Next
		curr.Next.Next = next

		if next != nil {
			curr = next
		}
	}
	curr.Next = otherList

	return mergeList
}

func mergeTwoLists2(list1 *ListNode, list2 *ListNode) *ListNode {
	dummy := &ListNode{}
	tail := dummy

	for list1 != nil && list2 != nil {
		if list1.Val < list2.Val {
			tail.Next = list1
			list1 = list1.Next
		} else {
			tail.Next = list2
			list2 = list2.Next
		}
		tail = tail.Next
	}

	if list1 != nil {
		tail.Next = list1
	}
	if list2 != nil {
		tail.Next = list2
	}

	return dummy.Next
}

// func reverseList(head *ListNode) *ListNode {
// 	if head == nil || head.Next == nil {
// 		return head
// 	}

// 	curr := head
// 	next := head.Next
// 	curr.Next = nil
// 	for next.Next != nil {
// 		tmp := next
// 		next = next.Next
// 		tmp.Next = curr
// 		curr = tmp
// 	}
// 	next.Next = curr
// 	return next
// }

func main() {
}
