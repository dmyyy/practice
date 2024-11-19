/*
You are given an integer array nums where each element nums[i] indicates your maximum jump length at that position.

Return true if you can reach the last index starting from index 0, or false otherwise.

Example 1:

Input: nums = [1,2,0,1,0]

Output: true
Explanation: First jump from index 0 to 1, then from index 1 to 3, and lastly from index 3 to 4.

Example 2:

Input: nums = [1,2,1,0,1]

Output: false
*/

// recursive brute force
/*
fn can_jump(nums: Vec<u32>) -> bool {
    // brute force recursive O(n^n) solution
    if nums.len() == 1 {
        return true;
    }

    jump_helper(0, &nums)
}

fn jump_helper(curr_idx: usize, nums: &Vec<u32>) -> bool {
    if curr_idx == nums.len() - 1 {
        return true;
    } else {
        if nums[curr_idx] == 0 {
            return false;
        }

        for i in 1..=nums[curr_idx] as usize {
            if jump_helper(curr_idx + i, &nums) {
                return true;
            }
        }
    }
    return false;
}
*/

// same recursive solution with a cache
/*
fn can_jump(nums: Vec<u32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let mut cache = vec![true; nums.len()];
    jump_helper(0, &nums, &mut cache)
}

fn jump_helper(curr_idx: usize, nums: &Vec<u32>, cache: &mut Vec<bool>) -> bool {
    if curr_idx == nums.len() - 1 {
        return true;
    } else {
        if nums[curr_idx] == 0 {
            cache[curr_idx] = false;
            return false;
        }

        for i in 1..=nums[curr_idx] as usize {
            if cache[curr_idx + i] && jump_helper(curr_idx + i, nums, cache) {
                return true;
            }
        }
    }
    return false;
}
*/

// greedy
fn can_jump(nums: Vec<u32>) -> bool {
    let mut goal = nums.len() - 1;

    for (idx, jump_len) in nums.into_iter().enumerate().rev().skip(1) {
        if idx + jump_len as usize >= goal {
            goal = idx;
        }
    }

    goal == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(can_jump(vec![1, 2, 0, 1, 0]), true);
        assert_eq!(can_jump(vec![1, 2, 1, 0, 1]), false);
    }
}
