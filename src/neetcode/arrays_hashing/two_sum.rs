/*
Given an array of integers nums and an integer target, return the indices i and j such that nums[i] + nums[j] == target and i != j.

You may assume that every input has exactly one pair of indices i and j that satisfy the condition.

Return the answer with the smaller index first.

Example 1:

Input:
nums = [3,4,5,6], target = 7

Output: [0,1]
Explanation: nums[0] + nums[1] == 7, so we return [0, 1].

Example 2:

Input: nums = [4,5,6], target = 10

Output: [0,2]
Example 3:

Input: nums = [5,5], target = 10

Output: [0,1]
*/


fn two_sum(nums: Vec<i32>, target: i32) -> [usize; 2] {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return [i, j];
            }
        }
    }
    unreachable!("invalid input");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(two_sum(vec![3, 4, 5, 6], 7), [0, 1]);
        assert_eq!(two_sum(vec![4, 5, 6], 10), [0, 2]);
        assert_eq!(two_sum(vec![5, 5], 10), [0, 1]);
    }
}
