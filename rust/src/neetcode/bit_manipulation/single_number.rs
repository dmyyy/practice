/*
You are given a non-empty array of integers nums. Every integer appears twice except for one.

Return the integer that appears only once.

You must implement a solution with
𝑂
(
𝑛
)
O(n) runtime complexity and use only
𝑂
(
1
)
O(1) extra space.

Example 1:

Input: nums = [3,2,3]

Output: 2
Example 2:

Input: nums = [7,6,6,7,8]

Output: 8
*/

fn single_number(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in nums {
        res ^= i;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(single_number(vec![3, 2, 3]), 2);
        assert_eq!(single_number(vec![7, 6, 6, 7, 8]), 8);
    }
}
