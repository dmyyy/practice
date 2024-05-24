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

// iter through grid until we find an area of land
// dfs in order to dicover all connected components
//

use std::collections::HashSet;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut island_count = 0;
    // visited land nodes in the grid indexed (column, row)
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, val) in row.iter().cloned().enumerate() {
            if visited.contains(&(c, r)) {
                // already visited as part of another island - continue
                continue;
            }

            if val == '0' {
                // water - continue
                continue;
            }

            // found a piece of land we haven't visited yet
            island_count += 1;
            visited.insert((c, r));

            // recursively find all pieces of land that are part of this unvisited island
            for (adj_c, adj_r) in get_unvisited_adjacent(c, r, &visited, &grid) {
                island_helper(adj_c, adj_r, &mut visited, &grid);
            }
        }
    }

    island_count
}

fn island_helper(
    c: usize,
    r: usize,
    mut visited: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) {
    if grid[r][c] == '0' {
        // found water - finish recursion
        return;
    } else {
        // found land - add to visited
        visited.insert((c, r));

        // check the other directions
        for (adj_c, adj_r) in get_unvisited_adjacent(c, r, visited, grid) {
            island_helper(adj_c, adj_r, &mut visited, &grid);
        }
    }
}

// given a column and a row return valid adjacent positions
fn get_unvisited_adjacent(
    c: usize,
    r: usize,
    visited: &HashSet<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    // check the other directions
    let mut adjacent = Vec::new();
    if c > 0 && !visited.contains(&(c - 1, r)) {
        adjacent.push((c - 1, r));
    }
    if r > 0 && !visited.contains(&(c, r - 1)) {
        adjacent.push((c, r - 1));
    }
    if c < grid[r].len() - 1 && !visited.contains(&(c + 1, r)) {
        adjacent.push((c + 1, r));
    }
    if r < grid.len() - 1 && !visited.contains(&(c, r + 1)) {
        adjacent.push((c, r + 1));
    }

    adjacent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_islands_test() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(num_islands(grid), 1);

        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(grid), 3);
    }
}
