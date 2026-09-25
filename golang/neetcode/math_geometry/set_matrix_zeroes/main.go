package main

func setZeroes(matrix [][]int) {
	// O(m+n) space complexity

	/*
		zeroRows := make(map[int]bool)
		zeroCols := make(map[int]bool)

		for i, row := range matrix {
			for j, col := range row {
				if col == 0 {
					zeroRows[i] = true
					zeroCols[j] = true
				}
			}
		}

		for i, row := range matrix {
			for j := range row {
				if zeroRows[i] || zeroCols[j] {
					matrix[i][j] = 0
				}
			}
		}
	*/

	// O(1) space complexity

	rowZero := false
	colZero := false
	for i, row := range matrix {
		for j, num := range row {
			if num == 0 {
				if i == 0 {
					rowZero = true
				}
				if j == 0 {
					colZero = true
				}

				matrix[0][j] = 0
				matrix[i][0] = 0
			}
		}
	}

	for i := 1; i < len(matrix); i++ {
		for j := 1; j < len(matrix[0]); j++ {
			if matrix[0][j] == 0 || matrix[i][0] == 0 {
				matrix[i][j] = 0
			}
		}
	}
	if rowZero {
		for i := range len(matrix[0]) {
			matrix[0][i] = 0
		}
	}
	if colZero {
		for i := range len(matrix) {
			matrix[i][0] = 0
		}
	}
}
