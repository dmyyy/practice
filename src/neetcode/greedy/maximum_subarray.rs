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

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // Naive O(n^2) solution w/ O(n) space
    /*
    let mut maxs = Vec::with_capacity(nums.len());
    for (i, _) in nums.iter().enumerate() {
        let mut max = i32::MIN;
        let mut curr = 0;
        for num in nums[i..].iter() {
            curr += num;
            if curr > max {
                max = curr;
            }
        }
        maxs.push(max);
    }

    let mut res = i32::MIN;
    for max in maxs {
        if max > res {
            res = max;
        }
    }

    res
    */

    // Naive O(n^2) with O(1) space
    /*
    let n = nums.len();
    let mut ans = i32::MIN;
    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += nums[j];
            ans = ans.max(sum);
        }
    }
    ans
    */

    // DP O(N)
    // total work to be done - n additions
    // at each level we add the

    let mut additions = nums.clone();
    for i in 0..(additions.len() - 1) {
        additions[i] += additions[i + 1];
    }

    // find the max subarray sum using additions
    let mut sum = i32::MIN;

    0

    // O(n) solution exists here
    // Divide and conquer solution exists here
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
