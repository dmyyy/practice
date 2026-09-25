package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	// first num in preorder is root
	//
	// everything to the left in inorder list is left side of tree

	if len(preorder) == 0 {
		return nil
	}

	root := &TreeNode{
		Val:   preorder[0],
		Left:  nil,
		Right: nil,
	}

	var leftInorderNodes []int
	var rightInorderNodes []int
	for i, v := range inorder {
		if v == preorder[0] {
			// found root
			leftInorderNodes = inorder[:i]
			if i != len(inorder)-1 {
				rightInorderNodes = inorder[(i + 1):]
			}
		}
	}
	if leftInorderNodes != nil {
		for _, v := range preorder {
			// if v ==

		}
	}
	if rightInorderNodes != nil {

	}

	return root
}

func main() {

}
