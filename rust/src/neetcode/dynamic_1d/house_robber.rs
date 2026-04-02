/*
You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

Example 1:

Input: nums = [1,2,3,1]
Output: 4
Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.
Example 2:

Input: nums = [2,7,9,3,1]
Output: 12
Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
Total amount you can rob = 2 + 9 + 1 = 12.
*/

use std::cmp::max;

/*
// exploring search space this way is O(2^n) - can think of it as a decision tree that we are exploring fully
fn rob(nums: Vec<i32>) -> i32 {
    let mut cache = vec![0; nums.len()];
    fn dfs(i: usize, nums: &[i32]) -> i32 {
        if i >= nums.len() - 1 {
            // last house robbed is either last house or second to last
            return 0;
        }

        // rob current house and skip the next one, or skip current one
        max(nums[i] + dfs(i + 2, nums), dfs(i + 1, nums))
    }

    dfs(0, nums.as_slice())
}
*/

// TODO: try to build a better intuition for why this works...

fn rob(nums: Vec<i32>) -> i32 {
    // base case
    // rob(0) = max(nums[0] + rob(2:n), rob(1:n))
    // rob(i) = max(nums[i] + rob(i+2:n), rob(i+1:n))

    let (mut rob1, mut rob2) = (0, 0);
    for n in nums {
        let temp = max(n + rob1, rob2);
        rob1 = rob2;
        rob2 = temp;
    }
    rob2
}
