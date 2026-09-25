package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrder(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}

	res := [][]int{}
	q := []*TreeNode{root}

	// bfs
	for len(q) > 0 {
		level := []int{}
		count := len(q)

		for i := 0; i < count; i++ {
			node := q[0]
			q = q[1:]
			level = append(level, node.Val)

			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		res = append(res, level)
	}
	return res
}

// func levelOrder(root *TreeNode) [][]int {
// if root == nil {
// 	return nil
// }
//
// // do a bfs - add nodes to the list
// levels := make([][]int, 0)
// levels = append(levels, []int{root.Val})
// levelOrderHelper(root, levels, 0)
// return levels
// }

// func levelOrderHelper(root *TreeNode, levels [][]int, depth int) [][]int {
// 	fmt.Println(levels)
// 	// bfs
// 	if root == nil {
// 		return nil
// 	} else {
// 		// length of levels >= depth
// 		if depth+1 >= len(levels) {
// 			// never before seen depth
// 			var vals []int
// 			if root.Left != nil {
// 				vals = append(vals, root.Left.Val)
// 			}
//
// 			if root.Right != nil {
// 				vals = append(vals, root.Right.Val)
// 			}
//
// 			levels = append(levels, vals)
// 		} else {
// 			// already seen depth
// 			levels[depth+1] = append(levels[depth+1], root.Left.Val, root.Right.Val)
// 			// if depth+1 < len(levels) {
// 			// 	fmt.Println(levels[depth+1])
// 			// 	levels[depth+1] = append(levels[depth+1], root.Left.Val, root.Right.Val)
// 			// } else {
// 			// 	levels = append(levels, []int{root.Left.Val, root.Right.Val})
// 			// }
// 		}
// 		levelOrderHelper(root.Left, levels, depth+1)
// 		levelOrderHelper(root.Right, levels, depth+1)
// 		return levels
// 	}
// }

func main() {
	// root := TreeNode{0, &TreeNode{1, 3, 4}, &TreeNode{2, nil, nil}}
	// fmt.Println(levelOrder(&root))
}
