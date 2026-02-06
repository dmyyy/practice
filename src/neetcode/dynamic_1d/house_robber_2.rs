/*
You are given an integer array nums where nums[i] represents the amount of money the ith house has. The houses are arranged in a circle, i.e. the first house and the last house are neighbors.

You are planning to rob money from the houses, but you cannot rob two adjacent houses because the security system will automatically alert the police if two adjacent houses were both broken into.

Return the maximum amount of money you can rob without alerting the police.

Example 1:

Input: nums = [3,4,3]

Output: 4
Explanation: You cannot rob nums[0] + nums[2] = 6 because nums[0] and nums[2] are adjacent houses. The maximum you can rob is nums[1] = 4.

Example 2:

Input: nums = [2,9,8,3,6]

Output: 15
Explanation: You cannot rob nums[0] + nums[2] + nums[4] = 16 because nums[0] and nums[4] are adjacent houses. The maximum you can rob is nums[1] + nums[4] = 15.

*/

// recursive -> memoized solution
use std::cmp::max;
fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    } else if nums.len() == 1 {
        // if there's only a single house is it considered next to itself? then we should return 0...
        return nums[0];
    }

    let mut cache = vec![-1; nums.len()];
    fn rob_recursive(curr: usize, end: usize, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if curr > end {
            0
        } else if curr == end {
            nums[curr]
        } else if cache[curr] != -1 {
            cache[curr]
        } else {
            max(
                // rob
                nums[curr] + rob_recursive(curr + 2, end, nums, cache),
                // skip and try robbing next house
                rob_recursive(curr + 1, end, nums, cache),
            )
        }
    }

    max(
        rob_recursive(0, nums.len() - 2, &nums, &mut cache),
        rob_recursive(1, nums.len() - 1, &nums, &mut cache),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rob_basic_api_surface_and_edge_cases() {
        assert_eq!(rob(vec![]), 0);
        assert_eq!(rob(vec![7]), 7);

        // Classic linear example (without circle would be 4) but circle blocks taking both ends.
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);

        // Circle-specific: cannot take both first and last.
        assert_eq!(rob(vec![2, 3, 2]), 3);

        // Prefer middle run.
        assert_eq!(rob(vec![1, 2, 3]), 3);

        // Larger case; best is 3 + 5 + 4 = 12 (cannot take both 2s at ends).
        assert_eq!(rob(vec![2, 3, 2, 5, 1, 4, 2]), 12);

        // Another circle constraint check: best is 10 + 10 = 20, not 10 + 10 + 1.
        assert_eq!(rob(vec![10, 1, 1, 10]), 11);
    }
}
