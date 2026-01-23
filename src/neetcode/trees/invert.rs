/*

Given the root of a binary tree, invert the tree, and return its root.

Example 1:

Input: root = [4,2,7,1,3,6,9]
Output: [4,7,2,9,6,3,1]

Example 2:

Input: root = [2,1,3]
Output: [2,3,1]

Example 3:

Input: root = []
Output: []

*/

use super::*;

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }

    todo!();
}

fn invert_tree_helper(root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(root_node) = root {
        // root_node.into_inner().left.replace()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;
//     use std::collections::VecDeque;
//     use std::rc::Rc;

//     fn rc_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
//         Some(Rc::new(RefCell::new(TreeNode::new(val))))
//     }

//     fn set_children(
//         parent: &Option<Rc<RefCell<TreeNode>>>,
//         left: Option<Rc<RefCell<TreeNode>>>,
//         right: Option<Rc<RefCell<TreeNode>>>,
//     ) {
//         if let Some(p) = parent {
//             let mut pb = p.borrow_mut();
//             pb.left = left;
//             pb.right = right;
//         }
//     }

//     fn level_order_vals(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
//         let mut res = Vec::new();
//         let mut q = VecDeque::new();
//         q.push_back(root);
//         while let Some(node_opt) = q.pop_front() {
//             match node_opt {
//                 None => res.push(None),
//                 Some(rc) => {
//                     let n = rc.borrow();
//                     res.push(Some(n.val));
//                     q.push_back(n.left.clone());
//                     q.push_back(n.right.clone());
//                 }
//             }
//         }
//         while res.last().map_or(false, |v| v.is_none()) {
//             res.pop();
//         }
//         res
//     }

//     #[test]
//     fn invert_tree_basic() {
//         let root: Option<Rc<RefCell<TreeNode>>> = None;
//         let inv = invert_tree(root.clone());
//         assert!(inv.is_none());

//         let one = rc_node(42);
//         let orig = one.clone();
//         let inv = invert_tree(one);
//         assert!(Rc::ptr_eq(inv.as_ref().unwrap(), orig.as_ref().unwrap()));
//         assert_eq!(level_order_vals(inv.clone()), vec![Some(42)]);

//         let n1 = rc_node(1);
//         let n2 = rc_node(2);
//         let n3 = rc_node(3);
//         let n4 = rc_node(4);
//         let n5 = rc_node(5);
//         let n6 = rc_node(6);
//         let n7 = rc_node(7);
//         set_children(&n1, n2.clone(), n3.clone());
//         set_children(&n2, n4.clone(), n5.clone());
//         set_children(&n3, n6.clone(), n7.clone());
//         let orig_top = n1.clone();
//         let inv = invert_tree(n1);
//         assert!(Rc::ptr_eq(
//             inv.as_ref().unwrap(),
//             orig_top.as_ref().unwrap()
//         ));
//         assert_eq!(
//             level_order_vals(inv.clone()),
//             vec![
//                 Some(1),
//                 Some(3),
//                 Some(2),
//                 Some(7),
//                 Some(6),
//                 Some(5),
//                 Some(4)
//             ]
//         );

//         let r = rc_node(1);
//         let l = rc_node(2);
//         let ll = rc_node(3);
//         set_children(&r, l.clone(), None);
//         set_children(&l, ll.clone(), None);
//         let expected_after = vec![Some(1), None, Some(2), None, Some(3)];
//         let inv1 = invert_tree(r.clone());
//         assert_eq!(level_order_vals(inv1.clone()), expected_after);
//         let inv2 = invert_tree(inv1);
//         assert_eq!(
//             level_order_vals(inv2),
//             vec![Some(1), Some(2), None, Some(3)]
//         );
//     }
// }
