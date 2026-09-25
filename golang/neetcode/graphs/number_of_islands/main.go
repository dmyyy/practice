package main

import "fmt"

func numIslands(grid [][]byte) int {
	count := 0
	// iterate through grid and
	// - if we find one that is land do dfs to explore around a little bit
	// - keep visited upated
	for i, bytes := range grid {
		for j, b := range bytes {
			if b == '1' {
				// found land perform dfs
				count += 1
				fmt.Println("do dfs on grid: ", grid)
				dfs(grid, j, i)
				fmt.Println("after dfs: ", grid)
			}
		}
	}
	return count
}

// index into slice with normal int
func dfs(grid [][]byte, x int, y int) {
	if y < 0 || y >= len(grid) || x < 0 || x >= len(grid[y]) || grid[y][x] != '1' {
		// out of bounds
		return
	}

	grid[y][x] = '2'

	dfs(grid, x-1, y)
	dfs(grid, x+1, y)
	dfs(grid, x, y-1)
	dfs(grid, x, y+1)
}

func main() {
	grid := [][]byte{{'1', '1', '1', '1', '0'}, {'1', '1', '0', '1', '0'}, {'1', '1', '0', '0', '0'}, {'0', '0', '0', '0', '0'}}
	fmt.Println(numIslands(grid))
}
