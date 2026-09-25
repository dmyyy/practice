package main

func pacificAtlantic(heights [][]int) [][]int {
	// start a dfs from every cell touching the ocean

	res := [][]int{}

	pacificVisited := [][]bool{}
	atlanticVisited := [][]bool{}

	m := len(heights)
	n := len(heights[0])

	pacificBorder := [][2]int{}
	atlanticBorder := [][2]int{}

	for i := range heights {
		pacificBorder = append(pacificBorder, [2]int{i, 0})
		atlanticBorder = append(atlanticBorder, [2]int{i, n - 1})
	}
	for j := range heights[0] {
		pacificBorder = append(pacificBorder, [2]int{0, j})
		atlanticBorder = append(atlanticBorder, [2]int{i, n - 1})
	}

	return res
}
