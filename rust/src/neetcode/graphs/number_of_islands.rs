/*

Given a 2D grid grid where '1' represents land and '0' represents water, count and return the number of islands.

An island is formed by connecting adjacent lands horizontally or vertically and is surrounded by water. You may assume water is surrounding the grid (i.e., all the edges are water).

Example 1:

Input: grid = [
    ["0","1","1","1","0"],
    ["0","1","0","1","0"],
    ["1","1","0","0","0"],
    ["0","0","0","0","0"]
  ]
Output: 1
Example 2:

Input: grid = [
    ["1","1","0","0","1"],
    ["1","1","0","0","1"],
    ["0","0","1","0","0"],
    ["0","0","0","1","1"]
  ]
Output: 4
*/

// problems
// - messed up i,j rows,cols in dfs
// - didn't explore up as well

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    // iter through grid
    // keep track of visited locations
    // when we find a piece of land
    // - increment island count
    // - explore surrounding land marking it as visited (using dfs)

    fn dfs(
        i: usize,
        j: usize,
        m: usize,
        n: usize,
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) {
        if i >= m || j >= n {
            // oob
            return;
        }
        if grid[i][j] == '0' || visited[i][j] {
            return;
        } else {
            // unvisited land - mark as visited
            visited[i][j] = true;
            // left
            if j >= 1 {
                dfs(i, j - 1, m, n, grid, visited)
            }
            // right
            if j < n - 1 {
                dfs(i, j + 1, m, n, grid, visited)
            }
            // down
            if i < m - 1 {
                dfs(i + 1, j, m, n, grid, visited)
            }
            // up
            if i >= 1 {
                dfs(i - 1, j, m, n, grid, visited)
            }
        }
    }

    let mut num_islands = 0;
    let (m, n) = (grid.len(), grid[0].len());
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' && !visited[i][j] {
                // land - explore recursively
                num_islands += 1;
                dfs(i, j, m, n, &grid, &mut visited);
            }
        }
    }
    num_islands
}
