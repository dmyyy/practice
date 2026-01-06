pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    solve_sudoku_helper(board, 0, 0);
}

pub fn solve_sudoku_helper(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
    // base case
    if board[8][8] != '.' {
        return;
    } else {
        // if it's already chosen - it's valid we don't care
        // recurse right away

        let mut valid_digits = [true; 9];

        // eliminate numbers in current row
        for c in board[row].iter() {
            valid_digits[c.to_digit(10).unwrap() as usize] = false;
        }

        // eliminate numbers in current col
        for row in 0..9 {
            let c = board[row][col];
            valid_digits[c.to_digit(10).unwrap() as usize] = false;
        }

        // verify
        // eliminate numbers in current 3x3 grid (double for loop) step by 3
        // TODO:
        // for {
        //     for {}
        // }
        //

        // valid numbers we have left
        // for digit in valid_possible_digits {
        //     // recurse
        // }
    }
}
