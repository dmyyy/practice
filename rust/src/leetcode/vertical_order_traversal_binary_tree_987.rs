/*
Given the root of a binary tree, calculate the vertical order traversal of the binary tree.

For each node at position (row, col), its left and right children will be at positions (row + 1, col - 1) and (row + 1, col + 1) respectively. The root of the tree is at (0, 0).

The vertical order traversal of a binary tree is a list of top-to-bottom orderings for each column index starting from the leftmost column and ending on the rightmost column. There may be multiple nodes in the same row and same column. In such a case, sort these nodes by their values.

Return the vertical order traversal of the binary tree.

Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: [[9],[3,15],[20],[7]]
Explanation:
Column -1: Only node 9 is in this column.
Column 0: Nodes 3 and 15 are in this column in that order from top to bottom.
Column 1: Only node 20 is in this column.
Column 2: Only node 7 is in this column.
Example 2:


Input: root = [1,2,3,4,5,6,7]
Output: [[4],[2],[1,5,6],[3],[7]]
Explanation:
Column -2: Only node 4 is in this column.
Column -1: Only node 2 is in this column.
Column 0: Nodes 1, 5, and 6 are in this column.
          1 is at the top, so it comes first.
          5 and 6 are at the same position (2, 0), so we order them by their value, 5 before 6.
Column 1: Only node 3 is in this column.
Column 2: Only node 7 is in this column.
Example 3:


Input: root = [1,2,3,4,6,5,7]
Output: [[4],[2],[1,5,6],[3],[7]]
Explanation:
This case is the exact same as example 2, but with nodes 5 and 6 swapped.
Note that the solution remains the same since 5 and 6 are in the same location and should be ordered by their values.
*/

use super::prelude::*;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

// - Option::as_ref - converts Option<T> to Option<&T> avoids transferring ownership on unwrap() (unwrap consumes self)
// - Deal w/ Rc<RefCell<T>> if possible
//   - to get a value from &TreePtr - need to do borrow()
//
// instead of and_modify.or_insert -> or_insert and work with collection directly

type TreePtr = Rc<RefCell<TreeNode>>;

fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn dfs(v: &TreePtr, row: i32, col: i32, ans: &mut BTreeMap<i32, Vec<(i32, i32)>>) {
        ans.entry(col)
            .or_insert(Vec::default())
            .push((row, v.borrow().val));
        let v_node = v.borrow();
        if v_node.left.is_some() {
            dfs(&v_node.left.as_ref().unwrap(), row + 1, col - 1, ans);
        }
        if v_node.right.is_some() {
            dfs(&v_node.right.as_ref().unwrap(), row + 1, col + 1, ans);
        }
    }

    // build up ans
    let mut ans = BTreeMap::default();
    dfs(&root.unwrap(), 0, 0, &mut ans);
    let mut res = Vec::new();
    for (col, vec) in &mut ans {
        vec.sort();
        let vec = vec.iter().map(|(_, v)| *v).collect::<Vec<i32>>();
        res.push(vec);
    }
    res
}
