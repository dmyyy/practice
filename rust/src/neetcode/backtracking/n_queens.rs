/*

The n-queens puzzle is the problem of placing n queens on an n x n chessboard so that no two queens can attack each other.

A queen in a chessboard can attack horizontally, vertically, and diagonally.

Given an integer n, return all distinct solutions to the n-queens puzzle.

Each solution contains a unique board layout where the queen pieces are placed. 'Q' indicates a queen and '.' indicates an empty space.

You may return the answer in any order.

Example 1:



Input: n = 4

Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
Explanation: There are two different solutions to the 4-queens puzzle.

Example 2:

Input: n = 1

Output: [["Q"]]

*/

// fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
//     // - construct some nxn board
//     // - start placing queens, and making constraints
//     //    - need to create n strings of length n

//     let n = n as usize;

//     let mut queens: Vec<(usize, usize)> = Vec::new();
//     let valid_positions = vec![vec![true; n]; n];
//     let mut res = Vec::new();

//     solve_n_queens_helper(queens, 1, &mut res);

//     res
// }

// fn solve_n_queens_helper(
//     queens: Vec<(usize, usize)>,
//     max_queens: usize,
//     mut valid_positions: Vec<Vec<bool>>,
//     res: &mut Vec<Vec<String>>,
// ) {
//     while let Some(pos) = valid_positions.iter().flatten().find(|v| **v) {
//         // try placing queen at valid pos and see if it intersects with any other queens?
//     }

//     if queens.count() == max_queens {
//         // found a new match - add to res
//     }
// }

// // serialize placed queens into a board
// fn to_board(n: usize, queens: Vec<(usize, usize)>) -> Vec<String> {
//     let mut board = vec![".".repeat(n); n];
//     for (x, y) in queens {
//         board[y].replace_range(x..x + 1, "Q");
//     }
//     board
// }
