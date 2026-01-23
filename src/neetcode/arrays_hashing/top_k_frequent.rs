/*
Given an integer array nums and an integer k, return the k most frequent elements within the array.

The test cases are generated such that the answer is always unique.

You may return the output in any order.

Example 1:

Input: nums = [1,2,2,3,3,3], k = 2

Output: [2,3]
Example 2:

Input: nums = [7,7], k = 1

Output: [7]
*/

use std::collections::{BinaryHeap, HashMap};

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // naive:
    // - create a counts map
    // -
    //

    let mut counts: HashMap<i32, i32> = HashMap::default();
    for n in nums {
        counts.entry(n).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut h: BinaryHeap<Item> = BinaryHeap::with_capacity(counts.capacity());
    for (&n, &count) in counts.iter() {
        h.push(Item { n, count });
    }

    let mut res = Vec::with_capacity(k as usize);
    for _ in 0..k {
        res.push(h.pop().unwrap().n);
    }

    res.sort();

    res
}

#[derive(Eq, PartialEq)]
struct Item {
    n: i32,
    count: i32,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(top_k_frequent(vec![1, 2, 2, 3, 3, 3], 2), vec![2, 3]);
        assert_eq!(top_k_frequent(vec![7, 7], 1), vec![7]);
    }
}
