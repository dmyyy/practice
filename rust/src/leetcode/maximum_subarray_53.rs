/*
Given an integer array nums, find the subarray with the largest sum, and return its sum.

Example 1:

Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: The subarray [4,-1,2,1] has the largest sum 6.
Example 2:

Input: nums = [1]
Output: 1
Explanation: The subarray [1] has the largest sum 1.
Example 3:

Input: nums = [5,4,-1,7,8]
Output: 23
Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
*/

fn max_sub_array(nums: Vec<i32>) -> i32 {
    // naive O(n^2)
    /*
    let mut max = 0;
    for i in 0..nums.len() {
        // max for sub-array starting at i
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            max = std::cmp::max(max, sum);
        }
    }
    max
    */

    let mut max = nums[0];
    let mut curr = 0;
    for i in 0..nums.len() {
        if curr < 0 {
            curr = 0;
        }
        curr += nums[i];
        max = std::cmp::max(max, curr)
    }
    max
}
