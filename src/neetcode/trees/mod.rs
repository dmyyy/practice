use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub mod test_helpers;
// pub use test_helpers::{build_tree, tree_to_vec};

mod diameter_tree;
mod invert;
mod max_depth_tree;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct BoxTreeNode {
    pub val: i32,
    pub left: Option<Box<BoxTreeNode>>,
    pub right: Option<Box<BoxTreeNode>>,
}

impl BoxTreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        BoxTreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl From<BoxTreeNode> for Option<Box<BoxTreeNode>> {
    fn from(value: BoxTreeNode) -> Self {
        Some(Box::new(value))
    }
}

// bst implementation: https://www.youtube.com/watch?v=yHi3q2Iiepc&list=PLb1VOxJqFzDdS-xV9OkKKPfXvtQ8y1Wzk&index=16

#[derive(Debug)]
pub struct BinarySearchTree {
    root: Option<Box<BoxTreeNode>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, val: i32) {
        // match &mut self.root {
        //     Some(root_node) => Self::insert_recursive(root_node, val),
        //     None => self.root = TreeNode::new(val).into(),
        // }

        self.insert_iterative(val);
    }

    fn insert_iterative(&mut self, val: i32) {
        if self.root.is_none() {
            self.root = BoxTreeNode::new(val).into();
            return;
        }

        let mut q = vec![self.root.as_mut().unwrap()];
        while let Some(node) = q.pop() {
            match val.cmp(&node.val) {
                std::cmp::Ordering::Less => {
                    let left_node = &mut node.left;
                    if let Some(left_node) = left_node {
                        q.push(left_node);
                    } else {
                        // found a place to insert new node
                        *left_node = BoxTreeNode::new(val).into();
                    }
                }
                std::cmp::Ordering::Greater => {
                    let right_node = &mut node.right;
                    if let Some(right_node) = right_node {
                        q.push(right_node);
                    } else {
                        // found a place to insert new node
                        *right_node = BoxTreeNode::new(val).into();
                    }
                }
                std::cmp::Ordering::Equal => println!("inserting duplicate value into BST"),
            }
        }
    }

    fn insert_recursive(root: &mut Box<BoxTreeNode>, val: i32) {
        match val.cmp(&root.val) {
            std::cmp::Ordering::Less => {
                if let Some(left_node) = &mut root.left {
                    Self::insert_recursive(left_node, val);
                } else {
                    // found a place to insert new node
                    root.left = BoxTreeNode::new(val).into();
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(right_node) = &mut root.right {
                    Self::insert_recursive(right_node, val);
                } else {
                    // found a place to insert new node
                    root.right = BoxTreeNode::new(val).into();
                }
            }
            std::cmp::Ordering::Equal => println!("inserting duplicate value into BST"),
        }
    }

    fn level_order(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut res = Vec::new();
        let mut q = VecDeque::new();
        q.push_front(self.root.as_ref().unwrap());
        while let Some(node) = q.pop_back() {
            res.push(node.val);
            if let Some(ref left) = node.left {
                q.push_front(left);
            }
            if let Some(ref right) = node.right {
                q.push_front(right);
            }
        }

        res
    }

    fn in_order_recursive(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut res = Vec::new();
        Self::in_order_helper(self.root.as_ref().unwrap(), &mut res);

        res
    }

    fn in_order_helper(root: &Box<BoxTreeNode>, res: &mut Vec<i32>) {
        if let Some(ref left) = root.left {
            Self::in_order_helper(left, res);
        }
        res.push(root.val);
        if let Some(ref right) = root.right {
            Self::in_order_helper(right, res);
        }

        // pre-order
        // res.push(root.val);
        // if let Some(ref left) = root.left {
        //     Self::in_order_helper(left, res);
        // }
        // if let Some(ref right) = root.right {
        //     Self::in_order_helper(right, res);
        // }

        // post-order
        // if let Some(ref left) = root.left {
        //     Self::in_order_helper(left, res);
        // }
        // if let Some(ref right) = root.right {
        //     Self::in_order_helper(right, res);
        // }
        // res.push(root.val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(10);
        bst.insert(3);
        bst.insert(1);
        bst.insert(6);
        bst.insert(4);

        assert!(bst.root.is_some());

        // println!("{bst:#?}");
        // println!("{:#?}", bst.level_order_traversal());
    }

    #[test]
    fn works_level_order() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(10);
        bst.insert(3);
        bst.insert(1);
        bst.insert(6);
        bst.insert(4);
        bst.insert(7);
        bst.insert(14);
        bst.insert(13);

        assert_eq!(bst.level_order(), vec![8, 3, 10, 1, 6, 14, 4, 7, 13]);
    }

    #[test]
    fn works_in_order() {
        let mut bst = BinarySearchTree::new();
        bst.insert(8);
        bst.insert(10);
        bst.insert(3);
        bst.insert(1);
        bst.insert(6);
        bst.insert(4);
        bst.insert(7);
        bst.insert(14);
        bst.insert(13);

        assert_eq!(bst.in_order_recursive(), vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
    }
}
