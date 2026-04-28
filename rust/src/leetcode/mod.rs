// Hard solutions list
// https://github.com/PavelSavchenkov/all-lc-hard-in-rust

mod facebook;

mod add_two_numbers_445;
mod fibonacci_number_509;
mod integer_to_english_words_273;
mod lru_cache_146;
mod maximum_subarray_53;
mod regular_expression_matching_10;
mod search_rotated_sorted_arr_281;
mod solve_sudoku;
mod tallest_billboard_956;
mod vertical_order_traversal_binary_tree_987;

mod prelude {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }
    }
}
