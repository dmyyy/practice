package main

func spiralOrder(matrix [][]int) []int {
	matrixLen := len(matrix) * len(matrix[0])
	var res []int

	left, right, top, bottom := 0, len(matrix[0])-1, 0, len(matrix)-1
	for len(res) < matrixLen {
		// right
		for col := left; col <= right; col++ {
			res = append(res, matrix[top][col])
		}
		// down
		for row := top + 1; row <= bottom; row++ {
			res = append(res, matrix[row][right])
		}

		// make sure we're on a diff row
		if top != bottom {
			// left
			for col := right - 1; col >= left; col-- {
				res = append(res, matrix[bottom][col])
			}
		}
		// make sure we're on a diff col
		if left != right {
			// up
			for row := bottom - 1; row > top; row-- {
				res = append(res, matrix[row][left])
			}
		}

		left++
		right--
		top++
		bottom--
	}
	return res

	// initial attempt - close + fixable but complicated and not as clean as it could be

	// row, col := 0, 0
	// matrixLen := len(matrix) * len(matrix[0])
	// res := make([]int, matrixLen)
	// visited := make(map[Pair]bool)

	// // currently goes in a spiral - but doesn't attempt to spiral in...

	// for len(visited) < matrixLen {
	// 	if visited[Pair{row, col}] {
	// 		col += 1
	// 	}

	// 	// right
	// 	for col < len(matrix[row])-1 && !visited[Pair{row, col}] {
	// 		res = append(res, matrix[row][col])
	// 		visited[Pair{row, col}] = true
	// 		col += 1
	// 	}
	// 	// down
	// 	for row < len(matrix)-1 && !visited[Pair{row, col}] {
	// 		res = append(res, matrix[row][col])
	// 		visited[Pair{row, col}] = true
	// 		row += 1
	// 	}
	// 	// left
	// 	for col > 0 && !visited[Pair{row, col}] {
	// 		res = append(res, matrix[row][col])
	// 		visited[Pair{row, col}] = true
	// 		col -= 1
	// 	}
	// 	// up
	// 	for row > 0 && !visited[Pair{row, col}] {
	// 		res = append(res, matrix[row][col])
	// 		visited[Pair{row, col}] = true
	// 		row -= 1
	// 	}
	// }
}
