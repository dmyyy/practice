package main

// only store stack of path to left-most node O(h) height

type BSTIterator struct {
	Stack []*TreeNode
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func Constructor(root *TreeNode) BSTIterator {
	var s []*TreeNode
	for root != nil {
		s = append(s, root)
		root = root.Left
	}
	return BSTIterator{
		Stack: s,
	}
}

func (this *BSTIterator) Next() int {
	next := this.Stack[len(this.Stack)-1]
	this.Stack = this.Stack[:len(this.Stack)-1]
	node := next.Right
	for node != nil {
		this.Stack = append(this.Stack, node)
		node = node.Left
	}
	return next.Val
}

func (this *BSTIterator) HasNext() bool {
	return len(this.Stack) != 0
}

func main() {
}
