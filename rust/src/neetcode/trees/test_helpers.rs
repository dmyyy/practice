// use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// use super::BoxTreeNode;

// pub fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
//     if vals.is_empty() {
//         return None;
//     }
//     let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
//     let mut queue = VecDeque::new();
//     queue.push_back(root.clone());
//     let mut i = 1;
//     while i < vals.len() {
//         if let Some(node) = queue.pop_front() {
//             if let Some(val) = vals[i] {
//                 let left = Rc::new(RefCell::new(TreeNode::new(val)));
//                 node.borrow_mut().left = Some(left.clone());
//                 queue.push_back(left);
//             }
//             i += 1;
//             if i < vals.len() {
//                 if let Some(val) = vals[i] {
//                     let right = Rc::new(RefCell::new(TreeNode::new(val)));
//                     node.borrow_mut().right = Some(right.clone());
//                     queue.push_back(right);
//                 }
//                 i += 1;
//             }
//         }
//     }
//     Some(root)
// }

// pub fn tree_to_vec(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
//     let mut result = Vec::new();
//     let mut queue = VecDeque::new();
//     if let Some(node) = root {
//         queue.push_back(Some(node.clone()));
//     } else {
//         return result;
//     }

//     while let Some(opt_node) = queue.pop_front() {
//         if let Some(node_rc) = opt_node {
//             let node_ref = node_rc.borrow();
//             result.push(Some(node_ref.val));
//             queue.push_back(node_ref.left.clone());
//             queue.push_back(node_ref.right.clone());
//         } else {
//             result.push(None);
//         }
//     }

//     // clean up extra Nones at the end
//     while result.last() == Some(&None) {
//         result.pop();
//     }
//     result
// }

// tree traversals - https://www.youtube.com/watch?v=4Vj0dH96xYg&list=PLb1VOxJqFzDdS-xV9OkKKPfXvtQ8y1Wzk&index=12

struct LevelTraversal {}

// impl Iterator for LevelTraversal {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
