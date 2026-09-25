package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	// dfs
	return maxDepthHelper(root, 1)
}

func maxDepthHelper(root *TreeNode, maxDepth int) int {
	if root == nil {
		// max depth was at the previous level
		return maxDepth - 1
	} else {
		// recursive case
		return max(maxDepthHelper(root.Left, maxDepth+1), maxDepthHelper(root.Right, maxDepth+1))
	}
}
