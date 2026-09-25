package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	// mostly works - doesn't handle overflow...?

	/* if l1 == nil {
		return l2
	} else if l2 == nil {
		return l1
	}

	curr := l1
	digit := 1
	num1 := 0
	for curr != nil {
		num1 += curr.Val * digit
		digit *= 10
		curr = curr.Next
	}

	curr = l2
	digit = 1
	num2 := 0
	for curr != nil {
		num2 += curr.Val * digit
		digit *= 10
		curr = curr.Next
	}

	var result *ListNode
	var num = num1 + num2
	if num == 0 {
		return &ListNode{
			0,
			nil,
		}
	}
	// create new linked list from num
	for num != 0 {
		onesDigit := num % 10
		node := &ListNode{onesDigit, nil}
		num /= 10

		if result == nil {
			result = node
			curr = result
		} else {
			curr.Next = node
			curr = curr.Next
		}
	}

	return result */

	// doesn't handle large ints correctly - correct solution uses elementary addition

	res := &ListNode{}
	tmp := res
	for l1 != nil || l2 != nil {
		if l1 != nil {
			tmp.Val += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			tmp.Val += l2.Val
			l2 = l2.Next
		}
		if tmp.Val > 9 {
			tmp.Val -= 10
			tmp.Next = &ListNode{Val: 1}
		} else if l1 != nil || l2 != nil {
			tmp.Next = &ListNode{}
		}
		tmp = tmp.Next
	}
	return res
}

func main() {

}
