package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func invertTree(root *TreeNode) *TreeNode {
	invertTreeHelper(root)
	return root
}

func invertTreeHelper(root *TreeNode) {
	if root == nil {
		return
	} else {
		temp := root.Left
		root.Left = root.Right
		root.Right = temp

		invertTreeHelper(root.Left)
		invertTreeHelper(root.Right)
	}
}

func main() {
}
