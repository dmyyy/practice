package main

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Pair struct {
	A *TreeNode
	B *TreeNode
}

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	if (root == nil && subRoot != nil) || (root != nil && subRoot == nil) {
		return false
	}

	q := []*TreeNode{root}
	for len(q) > 0 {
		node := q[0]
		q = q[1:]

		if node.Val == subRoot.Val {
			// dfs on both trees starting at current node
			s := []Pair{{node, subRoot}}
			isSubtree := true
			for len(s) > 0 {
				p := s[len(s)-1]
				s = s[:len(s)-1]

				// - A and B are either both nil or both non-nil
				// - have same value
				if (p.A == nil && p.B != nil) || (p.A != nil && p.B == nil) || p.A.Val != p.B.Val {
					isSubtree = false
					break
				}
				// - A.Left and B.Left are either both nil or both non-nil
				if (p.A.Left == nil && p.B.Left != nil) || (p.A.Left != nil && p.B.Left == nil) {
					isSubtree = false
					break
				}
				// - A.Right and B.Right are either both nil or both non-nil
				if (p.A.Right == nil && p.B.Right != nil) || (p.A.Right != nil && p.B.Right == nil) {
					isSubtree = false
					break
				}

				if p.A.Left != nil {
					s = append(s, Pair{p.A.Left, p.B.Left})
				}
				if p.A.Right != nil {
					s = append(s, Pair{p.A.Right, p.B.Right})
				}
			}

			if isSubtree {
				return true
			}
		}

		if node.Left != nil {
			q = append(q, node.Left)
		}
		if node.Right != nil {
			q = append(q, node.Right)
		}
	}

	return false
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	var isSame func(*TreeNode, *TreeNode) bool
	isSame = func(p *TreeNode, q *TreeNode) bool {
		if (p == nil && q != nil) || (q == nil && p != nil) {
			return false
		} else if p == nil && q == nil {
			return true
		} else {
			return p.Val == q.Val && isSame(p.Left, q.Left) && isSame(p.Right, q.Right)
		}
	}
	return isSame(p, q)
}

func main() {

}
