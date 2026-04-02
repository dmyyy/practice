/*
Given an integer array nums, return an array output where output[i] is the product of all the elements of nums except nums[i].

Each product is guaranteed to fit in a 32-bit integer.

Follow-up: Could you solve it in O(n) time without using the division operation?

Example 1:

Input: nums = [1,2,4,6]

Output: [48,24,12,8]
Example 2:

Input: nums = [-1,0,1,2,3]

Output: [0,-6,0,0,0]
*/

fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    let mut forward_products = nums.clone();
    forward_products[0] = nums[0];
    for idx in 1..nums.len() {
        forward_products[idx] *= forward_products[idx - 1];
    }

    let mut backward_products = nums.clone();
    backward_products[0] = nums[nums.len() - 1];
    for idx in (0..nums.len() - 1).rev() {
        backward_products[idx] *= backward_products[idx + 1];
    }

    for idx in 0..nums.len() {
        if idx == 0 {
            // only backwards
            nums[idx] = backward_products[idx + 1];
        } else if idx == nums.len() - 1 {
            nums[idx] = forward_products[idx - 1];
        } else {
            nums[idx] = forward_products[idx - 1] * backward_products[idx + 1];
        }
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = vec![1, 2, 4, 6];
        assert_eq!(vec![48, 24, 12, 8], product_except_self(input));

        input = vec![-1, 0, 1, 2, 3];
        assert_eq!(vec![0, -6, 0, 0, 0], product_except_self(input));
    }
}
