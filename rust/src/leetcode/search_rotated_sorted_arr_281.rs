/*
There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).

Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].

Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.

You must decrease the overall operation steps as much as possible.

Example 1:

Input: nums = [2,5,6,0,0,1,2], target = 0
Output: true
Example 2:

Input: nums = [2,5,6,0,0,1,2], target = 3
Output: false
*/

// FIXME: need to use an adapted version of binary search that handles rotation/duplicates

fn search(mut nums: Vec<i32>, target: i32) -> bool {
    // find partition point - sensitive to duplicates - doesn't really work...
    let (mut l, mut r) = (0, nums.len() - 1);
    while l <= r {
        let m = l + (r - l) / 2;

        if nums[m] > nums[l] {
            l = m + 1;
        } else {
            r = m;
        }
    }

    nums.rotate_left(l);

    nums.binary_search(&target).is_ok()
}
