/*
You are given a matrix grid where grid[i] is either a 0 (representing water) or 1 (representing land).

An island is defined as a group of 1's connected horizontally or vertically. You may assume all four edges of the grid are surrounded by water.

The area of an island is defined as the number of cells within the island.

Return the maximum area of an island in grid. If no island exists, return 0.

Example 1:



Input: grid = [
  [0,1,1,0,1],
  [1,0,1,0,1],
  [0,1,1,0,1],
  [0,1,0,0,1]
]

Output: 6
Explanation: 1's cannot be connected diagonally, so the maximum area of the island is 6.
*/

fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    // iterate through grid
    // if we find an island
    // - dfs
    // - update visited
    // - keep track of max size of island in dfs

    // m rows n cols
    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; n]; m];
    let mut max_area = 0;
    for row_idx in 0..m {
        for col_idx in 0..n {
            if grid[row_idx][col_idx] == 1 && !visited[row_idx][col_idx] {
                // found unvisited land - dfs and find size of island
                let mut area = 0;
                dfs(row_idx, col_idx, m, n, &grid, &mut visited, &mut area);
                max_area = std::cmp::max(max_area, area);
            }
        }
    }

    // explores island recursively and returns max size of island
    fn dfs(
        row_idx: usize,
        col_idx: usize,
        m: usize,
        n: usize,
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        area: &mut i32,
    ) {
        if row_idx >= m || col_idx >= n {
            // oob
            return;
        }
        if grid[row_idx][col_idx] == 0 || visited[row_idx][col_idx] {
            // water
            return;
        } else {
            // unvisited land
            visited[row_idx][col_idx] = true;
            *area += 1;
            // left
            if col_idx >= 1 {
                dfs(row_idx, col_idx - 1, m, n, grid, visited, area);
            }
            // right
            if col_idx < n - 1 {
                dfs(row_idx, col_idx + 1, m, n, grid, visited, area);
            }
            // down
            if row_idx < m - 1 {
                dfs(row_idx + 1, col_idx, m, n, grid, visited, area);
            }
            // up
            if row_idx >= 1 {
                dfs(row_idx - 1, col_idx, m, n, grid, visited, area);
            }
        }
    }
    max_area
}
