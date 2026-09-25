package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	pAncestors := dfs(root, p)
	qAncestors := dfs(root, q)

	// add pAncestors to a map
	pAncestorsMap := make(map[*TreeNode]bool, len(pAncestors))
	for _, n := range pAncestors {
		pAncestorsMap[n] = true
	}

	// find first shared ancestor
	for _, n := range qAncestors {
		if pAncestorsMap[n] {
			return n
		}
	}

	return nil
}

// find node in root tree using dfs
func dfs(root, node *TreeNode) []*TreeNode {
	if root == nil {
		return nil
	}
	if root == node {
		// found node - recurse up and built list of ancestors
		return []*TreeNode{node}
	} else {
		ancestors := dfs(root.Left, node)
		if len(ancestors) > 0 {
			return append(ancestors, root)
		}

		ancestors = dfs(root.Right, node)
		if len(ancestors) > 0 {
			return append(ancestors, root)
		}
		return nil
	}
}

func main() {

}
