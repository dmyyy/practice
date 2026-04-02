/*
Given a square n x n matrix of integers matrix, rotate it by 90 degrees clockwise.

You must rotate the matrix in-place. Do not allocate another 2D matrix and do the rotation.

Example 1:

Input: matrix = [
  [1,2],
  [3,4]
]

Output: [
  [3,1],
  [4,2]
]
Example 2:

Input: matrix = [
  [1,2,3],
  [4,5,6],
  [7,8,9]
]

Output: [
  [7,4,1],
  [8,5,2],
  [9,6,3]
]
*/

fn rotate(matrix: Vec<Vec<u32>>) {
    let n = matrix.len();
    for i in 0..n {
        // rotate n - 1 steps along the outside
    }

    for (i, row) in matrix.iter().enumerate() {
        // rotate n - 1 steps along the outside
        let temp = matrix[i][0];

        // matrix[i][matrix[i].len()] = ;
    }
}
