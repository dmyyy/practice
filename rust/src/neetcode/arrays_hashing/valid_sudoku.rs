/*
You are given a 9 x 9 Sudoku board board. A Sudoku board is valid if the following rules are followed:

Each row must contain the digits 1-9 without duplicates.
Each column must contain the digits 1-9 without duplicates.
Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without duplicates.
Return true if the Sudoku board is valid, otherwise return false

Note: A board does not need to be full or be solvable to be valid.
*/

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // check each row
    for row in 0..board.len() {
        let mut used_digits = [false; 10];
        for col in 0..board.len() {
            if let Some(digit) = board[row][col].to_digit(10) {
                if used_digits[digit as usize] {
                    // already set to true - duplicate value in the same row
                    return false;
                }
                used_digits[digit as usize] = true;
            }
        }
    }

    // check each col
    for col in 0..board.len() {
        let mut used_digits = [false; 10];
        for row in 0..board.len() {
            if let Some(digit) = board[row][col].to_digit(10) {
                if used_digits[digit as usize] {
                    // already set to true - duplicate value in the same row
                    return false;
                }
                used_digits[digit as usize] = true;
            }
        }
    }

    // check 3x3 boxes
    for row in (0..board.len()).step_by(3) {
        for col in (0..board.len()).step_by(3) {
            let mut used_digits = [false; 10];
            for i in row..(row + 3) {
                for j in col..(col + 3) {
                    if let Some(digit) = board[i][j].to_digit(10) {
                        if used_digits[digit as usize] {
                            // already set to true - duplicate value in the same row
                            return false;
                        }
                        used_digits[digit as usize] = true;
                    }
                }
            }
        }
    }

    true
}
