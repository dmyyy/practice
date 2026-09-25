package main

// dijkstras - answers shortest path from starting point to every other reachable point in graph
// we want - sp from every point to closest gate

func wallsAndGates(rooms [][]int) {
	// add all gates to queue
	// multi source bfs starting from all gates

	m, n := len(rooms), len(rooms[0])
	q := [][2]int{}

	// add gates to queue
	for i := range m {
		for j := range n {
			if rooms[i][j] == 0 {
				q = append(q, [2]int{i, j})
			}
		}
	}

	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for i := 0; i < len(q); i++ {
		r, c := q[i][0], q[i][1]

		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]

			if nr < 0 || nr >= m || nc < 0 || nc >= n || rooms[nr][nc] != 2147483647 {
				// out of bounds or not an empty room
				continue
			}

			rooms[nr][nc] = rooms[r][c] + 1
			q = append(q, [2]int{nr, nc})
		}
	}
}
