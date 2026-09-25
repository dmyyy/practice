package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	var validate func(*TreeNode, *int, *int) bool
	validate = func(root *TreeNode, min *int, max *int) bool {
		if root == nil {
			return true
		} else {
			if min != nil && root.Val <= *min {
				return false
			}
			if max != nil && root.Val >= *max {
				return false
			}

			return validate(root.Left, min, &root.Val) && validate(root.Right, &root.Val, max)
		}
	}

	return validate(root, nil, nil)

	// original incorrect solution
	// local invariants insufficient - need to check against a (min, max) range for each node

	/*
		// track seen values - bst cannot have duplicate values
		seen := make(map[int]bool)

		var validate func(*TreeNode) bool
		validate = func(root *TreeNode) bool {
			if root == nil {
				return true
			} else {
				alreadySeen := seen[root.Val]
				seen[root.Val] = true

				leftIsValid := true
				if root.Left != nil {
					leftIsValid = root.Left.Val < root.Val
				}
				rightIsValid := true
				if root.Right != nil {
					rightIsValid = root.Right.Val > root.Val
				}

				return !alreadySeen && leftIsValid && rightIsValid && validate(root.Left) && validate(root.Right)
			}
		}

		return validate(root)
	*/
}

func main() {

}
