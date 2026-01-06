/*

Given an unsorted array of integers nums and an integer k, return the kth largest element in the array.

By kth largest element, we mean the kth largest element in the sorted order, not the kth distinct element.

Follow-up: Can you solve it without sorting?

Example 1:

Input: nums = [2,3,1,5,4], k = 2

Output: 4
Example 2:

Input: nums = [2,3,1,1,5,5,4], k = 3

Output: 4

*/

use std::{any::Any, collections::BinaryHeap};

// Intuition is to use values as is and create a max-heap.
// Problem is that to get the kth-largest value you need an expensive O(n + klogn) operation which will be done on every add.
//
// Solution is to invert the values and keep the heap at len k - that way we can O(logn) to get the kth largest num.

struct KthLargest {
    k: i32,
    heap: BinaryHeap<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        assert!(k as usize <= nums.len());

        Self {
            k,
            heap: BinaryHeap::from(nums.into_iter().map(|n| -n).collect::<Vec<i32>>()),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(-val);

        while self.heap.len() > self.k as usize {
            self.heap.pop();
        }

        -*self.heap.peek().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn test_new_initializes_kth_largest_correctly() {
        let nums = vec![4, 5, 8, 2];
        let mut kth = KthLargest::new(2, nums);
        // The 2nd largest element in [4,5,8,2] is 5
        assert_eq!(kth.add(0), 5);
    }

    #[test]
    fn test_add_returns_correct_kth_largest_sequence() {
        let nums = vec![4, 5, 8, 2];
        let mut kth = KthLargest::new(3, nums);
        // Sorted descending: [8,5,4,2]
        // 3rd largest is 4
        assert_eq!(kth.add(3), 4); // [8,5,4,3,2]
        assert_eq!(kth.add(5), 5); // [8,5,5,4,3,2]
        assert_eq!(kth.add(10), 5); // [10,8,5,5,4,3,2]
        assert_eq!(kth.add(9), 8); // [10,9,8,5,5,4,3,2]
        assert_eq!(kth.add(4), 8); // [10,9,8,5,5,4,4,3,2]
    }

    #[test]
    fn test_heap_size_never_exceeds_k() {
        let nums = vec![1, 2, 3];
        let mut kth = KthLargest::new(2, nums);
        assert_eq!(kth.heap.len(), 3); // initialized with all elements negated

        kth.add(4);
        assert!(kth.heap.len() <= kth.k as usize);

        kth.add(5);
        assert!(kth.heap.len() <= kth.k as usize);
    }
}
