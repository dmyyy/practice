/*
Given the root of a binary tree, return its depth.

The depth of a binary tree is defined as the number of nodes along the longest path from the root node down to the farthest leaf node.

Example 1:

Input: root = [1,2,3,null,null,4]

Output: 3
Example 2:

Input: root = []

Output: 0
*/

use super::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut max_depth = 0;
    let mut s = Vec::new();
    s.push((root.unwrap().clone(), 0));
    while let Some((n, depth)) = s.pop() {
        if let Some(left) = n.borrow().left.clone() {
            s.push((left, depth + 1));
        }
        if let Some(right) = n.borrow().right.clone() {
            s.push((right, depth + 1));
        }

        // update max depth
        if depth > max_depth {
            max_depth = depth;
        }
    }
    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_single_node_tree() {
        let root = Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }));
        assert_eq!(max_depth(Some(root)), 0);
    }
}
