package main

import "fmt"

/*
You are given a a 9 x 9 Sudoku board board. A Sudoku board is valid if the following rules are followed:

Each row must contain the digits 1-9 without duplicates.
Each column must contain the digits 1-9 without duplicates.
Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without duplicates.
Return true if the Sudoku board is valid, otherwise return false

Note: A board does not need to be full or be solvable to be valid.

Example 1:

Input: board =
[["1","2",".",".","3",".",".",".","."],
 ["4",".",".","5",".",".",".",".","."],
 [".","9","8",".",".",".",".",".","3"],
 ["5",".",".",".","6",".",".",".","4"],
 [".",".",".","8",".","3",".",".","5"],
 ["7",".",".",".","2",".",".",".","6"],
 [".",".",".",".",".",".","2",".","."],
 [".",".",".","4","1","9",".",".","8"],
 [".",".",".",".","8",".",".","7","9"]]

Output: true
Example 2:

Input: board =
[["1","2",".",".","3",".",".",".","."],
 ["4",".",".","5",".",".",".",".","."],
 [".","9","1",".",".",".",".",".","3"],
 ["5",".",".",".","6",".",".",".","4"],
 [".",".",".","8",".","3",".",".","5"],
 ["7",".",".",".","2",".",".",".","6"],
 [".",".",".",".",".",".","2",".","."],
 [".",".",".","4","1","9",".",".","8"],
 [".",".",".",".","8",".",".","7","9"]]

Output: false
Explanation: There are two 1's in the top-left 3x3 sub-box.
*/

type Square struct {
	i int
	j int
}

// returns a square given some row, and col number
func square(i int, j int) Square {
	return Square{
		i / 3,
		j / 3,
	}
}

func isValidSudoku(board [][]string) bool {
	visited_squares := make(map[Square]map[string]bool)
	for i := range 3 {
		for j := range 3 {
			visited_squares[Square{
				i: i,
				j: j,
			}] = make(map[string]bool)
		}
	}

	visited_cols := make(map[int]map[string]bool)
	for i := range 9 {
		visited_cols[i] = make(map[string]bool)
	}

	for i, row := range board {
		visited_rows := make(map[string]bool)
		for j, s := range row {
			// rows
			if visited_rows[s] {
				return false
			} else {
				// not visited
				visited_rows[s] = true
			}

			// cols
			if visited_cols[j][s] {
				return false
			} else {
				// not visited
				visited_cols[j][s] = true
			}

			// squares
			if visited_squares[square(i, j)][s] {
				return false
			} else {
				// not visited
				visited_squares[square(i, j)][s] = true
			}

		}
	}

	return true
}

func main() {
	board := [][]string{
		{"1", "2", ".", ".", "3", ".", ".", ".", "."},
		{"4", ".", ".", "5", ".", ".", ".", ".", "."},
		{".", "9", "1", ".", ".", ".", ".", ".", "3"},
		{"5", ".", ".", ".", "6", ".", ".", ".", "4"},
		{".", ".", ".", "8", ".", "3", ".", ".", "5"},
		{"7", ".", ".", ".", "2", ".", ".", ".", "6"},
		{".", ".", ".", ".", ".", ".", "2", ".", "."},
		{".", ".", ".", "4", "1", "9", ".", ".", "8"},
		{".", ".", ".", ".", "8", ".", ".", "7", "9"},
	}
	fmt.Println(isValidSudoku(board))
}
