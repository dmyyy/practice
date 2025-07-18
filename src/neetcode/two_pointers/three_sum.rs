/*

Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] where nums[i] + nums[j] + nums[k] == 0, and the indices i, j and k are all distinct.

The output should not contain any duplicate triplets. You may return the output and the triplets in any order.

Example 1:

Input: nums = [-1,0,1,2,-1,-4]

Output: [[-1,-1,2],[-1,0,1]]
Explanation:
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
The distinct triplets are [-1,0,1] and [-1,-1,2].

Example 2:

Input: nums = [0,1,1]

Output: []
Explanation: The only possible triplet does not sum up to 0.

Example 3:

Input: nums = [0,0,0]

Output: [[0,0,0]]
Explanation: The only possible triplet sums up to 0.
*/

use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> HashSet<(i32, i32, i32)> {
    // sum of all 3 nums should be 0
    // brute force - check all combinations of triplets
    let mut res = HashSet::new();
    for i in 0..(nums.len() - 2) {
        for j in (i + 1)..(nums.len() - 1) {
            for k in (j + 1)..(nums.len()) {
                if nums[i] + nums[j] + nums[k] == 0 {
                    res.insert((nums[i], nums[j], nums[k]));
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            HashSet::from([(-1, -1, 2), (-1, 0, 1)])
        );
    }
}
