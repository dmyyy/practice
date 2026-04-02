use std::collections::HashMap;

/*
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Example 1:

Input: nums = [1,2,3,1]
Output: true
Example 2:

Input: nums = [1,2,3,4]
Output: false
Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]
Output: true
*/

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for num in nums {
        let count = counts.entry(num).or_default();
        *count += 1;
        if *count >= 2 {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
