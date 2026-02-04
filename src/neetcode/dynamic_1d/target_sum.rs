/*
You are given an array of integers nums and an integer target.

For each number in the array, you can choose to either add or subtract it to a total sum.

For example, if nums = [1, 2], one possible sum would be "+1-2=-1".
If nums=[1,1], there are two different ways to sum the input numbers to get a sum of 0: "+1-1" and "-1+1".

Return the number of different ways that you can build the expression such that the total sum equals target.

Example 1:

Input: nums = [2,2,2], target = 2

Output: 3
Explanation: There are 3 different ways to sum the input numbers to get a sum of 2.
+2 +2 -2 = 2
+2 -2 +2 = 2
-2 +2 +2 = 2
*/

use std::collections::HashMap;

fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    fn ts(
        sum: i32,
        target: i32,
        idx: usize,
        nums: &Vec<i32>,
        cache: &mut HashMap<(i32, usize, char), i32>,
    ) -> i32 {
        // base case: idx == nums.len() && sum == target
        if idx == nums.len() {
            if sum == target { 1 } else { 0 }
        } else {
            let c_plus = if let Some(&c) = cache.get(&(sum, idx, '+')) {
                c
            } else {
                let c = ts(sum + nums[idx], target, idx + 1, nums, cache);
                cache.insert((sum, idx, '+'), c);
                c
            };
            let c_minus = if let Some(&c) = cache.get(&(sum, idx, '-')) {
                c
            } else {
                let c = ts(sum - nums[idx], target, idx + 1, nums, cache);
                cache.insert((sum, idx, '-'), c);
                c
            };
            c_plus + c_minus
        }
    }
    // (sum, idx, op) -> num. combinations
    let mut cache: HashMap<(i32, usize, char), i32> = HashMap::new();
    ts(0, target, 0, &nums, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, find_target_sum_ways(vec![2, 2, 2], 2));
        // assert_eq!(2, find_target_sum_ways(vec![1, 1], 0));
    }

    #[test]
    fn covers_basic_api_surface() {
        assert_eq!(2, find_target_sum_ways(vec![1, 1], 0));
        assert_eq!(5, find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
        assert_eq!(8, find_target_sum_ways(vec![0, 0, 0], 0));
        assert_eq!(0, find_target_sum_ways(vec![1, 2, 3], 7));
        assert_eq!(1, find_target_sum_ways(vec![], 0));
        assert_eq!(0, find_target_sum_ways(vec![], 1));
    }
}
