package main

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Neighbors []*Node
 * }
 */

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	if node == nil {
		return nil
	}

	oldToNew := make(map[*Node]*Node, 0)
	oldToNew[node] = &Node{node.Val, make([]*Node, 0)}
	queue := make([]*Node, 0)
	queue = append(queue, node)

	for len(queue) != 0 {
		// fmt.Println(queue)
		// fmt.Println(oldToNew)
		// get curr and move queue one step forward
		curr := queue[0]
		queue = queue[1:]

		for _, neighbor := range curr.Neighbors {
			if _, ok := oldToNew[neighbor]; !ok {
				oldToNew[neighbor] = &Node{node.Val, make([]*Node, 0)}
				queue = append(queue, neighbor)
			}
			oldToNew[curr].Neighbors = append(oldToNew[curr].Neighbors, oldToNew[neighbor])
		}
	}

	return oldToNew[node]
}

// func cloneGraph(node *Node) *Node {
// 	// use bfs to traverse the graph and build it up as we go
// 	// usually you pair bfs traversal with a visited map

// 	parent := nil
// 	newNode:= Node{node.Val, nil}

// 	visited := make(map[*Node]bool)
// 	queue := make(*Node, 0)
// 	queue = append(queue, node)
// 	for len(queue) != 0 {
// 		for i := 0; i < len(queue); i++ {
// 			n := queue[i]

// 			if parent != nil {

// 			}

// 			newNode.Neighbors = make([]*Node, len(n.Neighbors))
// 			queue = append(queue, n.Neighbors...)
// 			// new root neighbors must be point to new nodes
// 			newRoot.Neighbors =

// 		}

// 	}

// 	// need to recursively update neighbors? that seems tricky
// 	newRoot := Node{node.Val, node

// }

func main() {

}
