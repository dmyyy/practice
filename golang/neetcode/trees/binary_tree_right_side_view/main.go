package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rightSideView(root *TreeNode) []int {
	// kind of like a post-order traversal (but only one node per-level)
	// map level -> node
	// do a dfs - keep track of the last seen node for each level

	// alternatively: bfs and only store the last node

	if root == nil {
		return nil
	}

	q := []*TreeNode{root}
	res := []int{}
	for len(q) > 0 {
		levelNodeCount := len(q)
		var node *TreeNode

		for range levelNodeCount {
			node = q[0]
			q = q[1:]

			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}

		res = append(res, node.Val)
	}

	return res
}

func main() {

}
