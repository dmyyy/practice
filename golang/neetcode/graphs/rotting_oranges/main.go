package main

// cleaner solution - using level order bfs
// needed a fresh > 0 on q loop - possible for last rotting oranges added to add +1 to mins even
// when there are no more fresh oranges to rot

func orangesRotting(grid [][]int) int {
	// multi source bfs from rotting oranges + keeping track of the time

	m, n := len(grid), len(grid[0])
	q := [][2]int{}

	// add all rotting oranges to q
	freshOranges := 0
	for i := range m {
		for j := range n {
			if grid[i][j] == 1 {
				freshOranges += 1
			}
			if grid[i][j] == 2 {
				q = append(q, [2]int{i, j})
			}
		}
	}

	// terminate if after a minute no fresh oranges are added to q
	mins := 0
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(q) > 0 && freshOranges > 0 {
		size := len(q)
		for i := range size {
			r, c := q[i][0], q[i][1]

			for _, d := range dirs {
				nr, nc := r+d[0], c+d[1]

				// check if oob
				if nr < 0 || nr >= m || nc < 0 || nc >= n {
					continue
				}

				if grid[nr][nc] == 1 {
					grid[nr][nc] = 2
					freshOranges--
					q = append(q, [2]int{nr, nc})
				}
			}
		}
		q = q[size:]
		mins++
	}

	if freshOranges != 0 {
		// not all oranges rotted
		return -1
	}
	return mins
}

// multi-source bfs
// appending to q in levels was tricky
// nil to zero out worked surprisingly

/* func orangesRotting(grid [][]int) int {
	// multi source bfs from rotting oranges + keeping track of the time

	m, n := len(grid), len(grid[0])
	q := [][2]int{}

	// add all rotting oranges to q
	freshOranges := 0
	for i := range m {
		for j := range n {
			if grid[i][j] == 1 {
				freshOranges += 1
			}
			if grid[i][j] == 2 {
				q = append(q, [2]int{i, j})
			}
		}
	}

	// terminate if after a minute no fresh oranges are added to q
	mins := 0
	freshlyRotten := [][2]int{}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(q) != 0 {
		r, c := q[0][0], q[0][1]
		q = q[1:]

		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]

			// check if oob
			if nr < 0 || nr >= m || nc < 0 || nc >= n {
				continue
			}

			if grid[nr][nc] == 1 {
				grid[nr][nc] = 2
				freshOranges--
				freshlyRotten = append(freshlyRotten, [2]int{nr, nc})
			}
		}

		if len(q) == 0 && len(freshlyRotten) != 0 {
			// propagated all rotting oranges to fresh oranges
			mins++
			q = append(q, freshlyRotten...)
			freshlyRotten = [][2]int{}
		}
	}

	if freshOranges != 0 {
		// not all oranges rotted
		return -1
	}
	return mins
} */
