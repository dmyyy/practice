package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func goodNodes(root *TreeNode) int {
	var goodNodesHelper func(*TreeNode, int) int
	goodNodesHelper = func(root *TreeNode, maxSoFar int) int {
		if root == nil {
			return 0
		} else {
			// good if node is the biggest in path
			isGood := true
			if maxSoFar > root.Val {
				// larger ancestor
				isGood = false
			}

			maxSoFar = max(maxSoFar, root.Val)
			if isGood {
				return 1 + goodNodesHelper(root.Left, maxSoFar) + goodNodesHelper(root.Right, maxSoFar)
			}
			return goodNodesHelper(root.Left, maxSoFar) + goodNodesHelper(root.Right, maxSoFar)
		}
	}

	return goodNodesHelper(root, root.Val)

	// var goodNodesHelper func(*TreeNode, []int) int
	// goodNodesHelper = func(root *TreeNode, path []int) int {
	// 	if root == nil {
	// 		return 0
	// 	} else {
	// 		// good if node is the biggest in path
	// 		isGood := true
	// 		for _, v := range path {
	// 			if v > root.Val {
	// 				// larger ancestor
	// 				isGood = false
	// 			}
	// 		}

	// 		if isGood {
	// 			return 1 + goodNodesHelper(root.Left, append(path, root.Val)) + goodNodesHelper(root.Right, append(path, root.Val))
	// 		}
	// 		return goodNodesHelper(root.Left, append(path, root.Val)) + goodNodesHelper(root.Right, append(path, root.Val))
	// 	}
	// }

	// var path []int
	// return goodNodesHelper(root, path)
}

func main() {

}
