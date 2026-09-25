package main

import (
	"fmt"
)

type Tree struct {
	X int
	L *Tree
	R *Tree
}

func dfs(node *Tree, leaf_id int, nodes []*Tree) *Tree {
	if node == nil {
		nodes = nodes[:len(nodes)-1]
		return nil
	}

	if node.X == leaf_id {
		// found value - use nodesFromRoot to change around pointers as we go back up the tree to the root and return
		prevNode := node
		for len(nodes) != 0 {
			parent := nodes[len(nodes)-1]
			nodes = nodes[:len(nodes)-1]

			if prevNode.L == nil {
				prevNode.L = parent
			} else if prevNode.R == nil {
				prevNode.R = parent
			}

			// points to a node that now points to parent - remove pointer inside
			if parent.L == prevNode {
				parent.L = nil
			}
			if parent.R == prevNode {
				parent.L = nil
			}

			prevNode = parent
		}

		// return new root
		return node
	}

	nodes = append(nodes, node)
	node = dfs(node.L, leaf_id, nodes)
	if node != nil {
		return node
	}

	nodes = append(nodes, node)
	node = dfs(node.R, leaf_id, nodes)
	if node != nil {
		return node
	}

	// unable to find node in both sides of the tree - panic
	panic("unreachable")
}

func rootedTree(root *Tree, leaf_id int) *Tree {
	return dfs(root, leaf_id, make([]*Tree, 0))

	stack := []*Tree{root}
	nodesFromRoot := []*Tree{}

	// perform dfs
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if node.X == leaf_id {
			// found value - use nodesFromRoot to change around pointers as we go back up the tree to the root and return
			prevNode := node
			for len(nodesFromRoot) != 0 {
				parent := nodesFromRoot[len(nodesFromRoot)-1]
				nodesFromRoot = nodesFromRoot[:len(nodesFromRoot)-1]

				if prevNode.L == nil {
					prevNode.L = parent
				} else if prevNode.R == nil {
					prevNode.R = parent
				}

				// points to a node that now points to parent - remove pointer inside
				if parent.L == prevNode {
					parent.L = nil
				}
				if parent.R == prevNode {
					parent.L = nil
				}

				prevNode = parent
			}

			// return new root
			return node
		}

		nodesFromRoot = append(nodesFromRoot, node)
		if node.R == nil && node.L == nil {
			// dead end
			nodesFromRoot = nodesFromRoot[:len(nodesFromRoot)-1]
		}

		// add right side first
		if node.R != nil {
			stack = append(stack, node.R)
		}
		if node.L != nil {
			stack = append(stack, node.L)
		}
	}

	return root
}

func main() {
	root := &Tree{
		X: 4,
		L: &Tree{
			2,
			nil,
			nil,
		},
		R: &Tree{
			8,
			nil,
			nil,
		},
	}

	res := rootedTree(root, 2)

	fmt.Printf("original tree: %v\n", root)
	fmt.Printf("modified tree: res: %v, res.L: %v\n", res, res.L)
}
