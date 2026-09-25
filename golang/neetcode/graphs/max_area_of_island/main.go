package main

import "fmt"

func maxAreaOfIsland(grid [][]int) int {
	// iterate through the grid
	// if we find an island, we explore it via dfs and mark places as explored and count size of island while we're at it
	//

	max := -1
	for y, row := range grid {
		for x, i := range row {
			if i == 1 {
				// explore via dfs
				area := dfs(grid, x, y, 1)
				if area > max {
					max = area
				}
			}
		}
	}
	return max
}

func dfs(grid [][]int, x int, y int, count int) int {
	if y < 0 || x < 0 || y >= len(grid) || x >= len(grid[y]) || grid[y][x] != 1 {
		return count - 1
	}

	grid[y][x] = 2

	count = dfs(grid, x-1, y, count+1)
	count = dfs(grid, x+1, y, count+1)
	count = dfs(grid, x, y-1, count+1)
	count = dfs(grid, x, y+1, count+1)
	return count
}

func main() {
	grid := [][]int{{0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0}, {0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0}, {0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0}}
	fmt.Println(maxAreaOfIsland(grid))
}
