package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func kthSmallest(root *TreeNode, k int) int {
	seen := 0
	res := 0

	var dfs func(*TreeNode, int)
	dfs = func(root *TreeNode, k int) {
		if root == nil {
			return
		} else {
			dfs(root.Left, k)

			// in-order traversal
			seen += 1
			if seen == k {
				res = root.Val
			}

			dfs(root.Right, k)
		}
	}

	dfs(root, k)
	return res
}

func main() {

}
