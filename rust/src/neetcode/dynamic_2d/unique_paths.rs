/*
There is an m x n grid where you are allowed to move either down or to the right at any point in time.

Given the two integers m and n, return the number of possible unique paths that can be taken from the top-left corner of the grid (grid[0][0]) to the bottom-right corner (grid[m - 1][n - 1]).

You may assume the output will fit in a 32-bit integer.

Example 1:



Input: m = 3, n = 6

Output: 21
Example 2:

Input: m = 3, n = 3

Output: 6
*/

use std::collections::{HashMap, HashSet};

// bottom up dp solution
fn unique_paths(m: u32, n: u32) -> u32 {
    if m == 1 || n == 1 {
        return 1;
    }

    // pos -> number of unique paths from pos. to (0, 0)
    let mut map = HashMap::<(u32, u32), u32>::new();
    map.insert((0, 0), 1);

    // positions we're currently evaluating on the diagonal
    let mut positions = HashSet::new();
    positions.insert((0, 1));
    positions.insert((1, 0));

    let end = (n - 1, m - 1);
    loop {
        // update path cache
        let mut new_positions = Vec::new();
        for pos in positions.iter() {
            let mut above = None;
            if pos.1 > 0 {
                above = map.get(&(pos.0, pos.1 - 1)).cloned();
            }

            let mut left = None;
            if pos.0 > 0 {
                left = map.get(&(pos.0 - 1, pos.1)).cloned();
            }

            match (above, left) {
                (Some(a), Some(b)) => map.insert(*pos, a + b),
                (Some(a), None) | (None, Some(a)) => map.insert(*pos, a),
                (None, None) => unreachable!("should never have no tiles above and to the left"),
            };

            if pos.0 < n - 1 {
                new_positions.push((pos.0 + 1, pos.1));
            }
            if pos.1 < m - 1 {
                new_positions.push((pos.0, pos.1 + 1));
            }
        }

        if positions.contains(&end) {
            return *map.get(&end).unwrap();
        }

        new_positions.dedup();
        positions.clear();
        positions.extend(new_positions);
    }
}

// recursive -> top down memo
fn unique_paths2(m: u32, n: u32) -> u32 {
    fn up(x: u32, y: u32, m: u32, n: u32, memo: &mut HashMap<(u32, u32), u32>) -> u32 {
        if let Some(&v) = memo.get(&(x, y)) {
            return v;
        }
        if x == m - 1 && y == n - 1 {
            return 1;
        }

        let mut res = 0;

        if x < m - 1 {
            // right
            res += up(x + 1, y, m, n, memo);
        }
        if y < n - 1 {
            // down
            res += up(x, y + 1, m, n, memo);
        }

        memo.insert((x, y), res);
        res
    }

    let mut memo = HashMap::new();
    up(0, 0, m, n, &mut memo)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(unique_paths(3, 6), 21);
        // assert_eq!(unique_paths(3, 3), 6);
        assert_eq!(unique_paths2(3, 2), 3);
        assert_eq!(unique_paths2(3, 6), 21);
        assert_eq!(unique_paths2(3, 3), 6);
    }
}
