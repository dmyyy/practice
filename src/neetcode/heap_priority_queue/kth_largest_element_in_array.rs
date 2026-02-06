/*
Given an integer array nums and an integer k, return the kth largest element in the array.

Note that it is the kth largest element in the sorted order, not the kth distinct element.

Can you solve it without sorting?

Example 1:

Input: nums = [3,2,1,5,6,4], k = 2
Output: 5
Example 2:

Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
Output: 4
*/

use std::collections::BinaryHeap;

fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut h: BinaryHeap<i32> = BinaryHeap::with_capacity(nums.len());
    h.extend(nums.iter());
    while k > 1 {
        h.pop();
        k -= 1;
    }
    h.pop().unwrap()
}
