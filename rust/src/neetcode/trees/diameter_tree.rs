// /*

// The diameter of a binary tree is defined as the length of the longest path between any two nodes within the tree. The path does not necessarily have to pass through the root.

// The length of a path between two nodes in a binary tree is the number of edges between the nodes. Note that the path can not include the same node twice.

// Given the root of a binary tree root, return the diameter of the tree.

// Example 1:

// Input: root = [1,null,2,3,4,5]

// Output: 3
// Explanation: 3 is the length of the path [1,2,3,5] or [5,3,2,4].

// Example 2:

// Input: root = [1,2,3]

// Output: 2

// */

// use super::TreeNode;

// use std::cell::RefCell;
// use std::rc::Rc;

// fn diameter_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     // brute force
//     // find the path from every node in the tree to every other node in the tree
//     // choose the longest path

//     // a non-leaf/non-root node cannot be the start/end of the longest path (because they trivially
//     // have a child/parent node which if taken can result in a longer path)
//     //
//     //
//     // 
//     // at each node store the longest path from that node down
//     // 

//     if let Some(ref n) = root {
//         let left = n.borrow().left.clone();
//         let right = n.borrow().right.clone();

//         let mut n = n.borrow_mut();
//         n.left = invert_tree(right);
//         n.right = invert_tree(left);
//     }

//     0
// }
