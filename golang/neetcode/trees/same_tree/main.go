package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	var isSame func(*TreeNode, *TreeNode) bool
	isSame = func(p *TreeNode, q *TreeNode) bool {
		if (p == nil && q != nil) || (q == nil && p != nil) {
			return false
		} else if p == nil && q == nil {
			return true
		} else {
			return p.Val == q.Val && isSame(p.Left, q.Left) && isSame(p.Right, q.Right)
		}
	}
	return isSame(p, q)
}

func main() {

}
