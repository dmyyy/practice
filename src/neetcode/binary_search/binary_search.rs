/*
Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

You must write an algorithm with O(log n) runtime complexity.

Example 1:

Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4

Example 2:

Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1
*/

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut top = nums.len() - 1;
    let mut bot = 0;
    let mut curr = (top + bot) / 2;
    loop {
        match nums[curr].cmp(&target) {
            std::cmp::Ordering::Less => {
                bot = curr + 1;
            }
            std::cmp::Ordering::Greater => {
                top = curr;
            }
            std::cmp::Ordering::Equal => {
                return curr as i32;
            }
        }
        let old_curr = curr;
        curr = (top + bot) / 2;
        if old_curr == curr {
            return -1;
        }
    } 
}

mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(search(vec![-1,0,3,5,9,12], 9), 4);
        assert_eq!(search(vec![-1,0,3,5,9,12], 2), -1);
        assert_eq!(search(vec![5], 5), 0);
        assert_eq!(search(vec![2, 5], 5), 1);
    }
}