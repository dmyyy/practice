// avoid enumerate and borrowing - depend on raw indexing
// using & in for loops on the left-hand side turns things into owned types and causes moves to happen
// stick with not moving things if you want to pass references to them in functions later...
//
// cleared visited as an optimization - but want to just revert everything back to false instead of clearing...
// -

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn dfs(
        board: &Vec<Vec<char>>,
        i: usize,
        j: usize,
        chars: &[u8],
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if chars.is_empty() {
            return true;
        } else {
            // up
            if let Some(i) = i.checked_sub(1)
                && !visited[i][j]
                && board[i][j] as u8 == chars[0]
            {
                visited[i][j] = true;
                if dfs(board, i, j, &chars[1..], visited) {
                    return true;
                }
                visited[i][j] = false;
            }
            // down
            if (i + 1) < board.len() && !visited[i + 1][j] && board[i + 1][j] as u8 == chars[0] {
                visited[i + 1][j] = true;
                if dfs(board, i + 1, j, &chars[1..], visited) {
                    return true;
                }
                visited[i + 1][j] = false;
            }
            // left
            if let Some(j) = j.checked_sub(1)
                && !visited[i][j]
                && board[i][j] as u8 == chars[0]
            {
                visited[i][j] = true;
                if dfs(board, i, j, &chars[1..], visited) {
                    return true;
                }
                visited[i][j] = false;
            }
            // right
            if (j + 1) < board[0].len() && !visited[i][j + 1] && board[i][j + 1] as u8 == chars[0] {
                visited[i][j + 1] = true;
                if dfs(board, i, j + 1, &chars[1..], visited) {
                    return true;
                }
                visited[i][j + 1] = false;
            }
        }
        false
    }

    let chars = word.as_bytes();
    let first = chars[0];
    let (mut i, mut j) = (0, 0);
    for row in board.iter() {
        for c in row {
            if *c as u8 == first {
                // attempt dfs
                // TODO: can optimize if we re-use visited map
                let mut visited = vec![vec![false; board[0].len()]; board.len()];
                visited[i][j] = true;
                if dfs(&board, i, j, &chars[1..], &mut visited) {
                    return true;
                }
                visited[i][j] = false;
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    false
}
