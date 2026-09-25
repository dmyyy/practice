package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isBalanced(root *TreeNode) bool {
	var isBalancedHelper func(*TreeNode) (int, bool)
	isBalancedHelper = func(node *TreeNode) (int, bool) {
		if node == nil {
			return 0, true
		} else {
			left, leftBalanced := isBalancedHelper(node.Left)
			right, rightBalanced := isBalancedHelper(node.Right)
			isBalanced := abs(left-right) <= 1

			return max(left, right) + 1, leftBalanced && rightBalanced && isBalanced
		}
	}

	_, isBalanced := isBalancedHelper(root)
	return isBalanced
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
}
