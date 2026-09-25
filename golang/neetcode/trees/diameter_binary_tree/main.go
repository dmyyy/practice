package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func diameterOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}
	maxDiameter := 0
	maxDiameterP := &maxDiameter
	diameterOfBinaryTreeHelper(root, maxDiameterP)
	return maxDiameter
}

func diameterOfBinaryTreeHelper(root *TreeNode, maxDiameterP *int) int {
	if root.Left == nil && root.Right == nil {
		return 1
	} else {
		leftPath := 0
		if root.Left != nil {
			leftPath = diameterOfBinaryTreeHelper(root.Left, maxDiameterP)
		}
		rightPath := 0
		if root.Right != nil {
			rightPath = diameterOfBinaryTreeHelper(root.Right, maxDiameterP)
		}

		*maxDiameterP = max(*maxDiameterP, leftPath+rightPath)

		return max(leftPath, rightPath) + 1
	}
}

func main() {
}
