/*
You are given an array of integers nums, where nums[i] represents the maximum length of a jump towards the right from index i. For example, if you are at nums[i], you can jump to any index i + j where:

j <= nums[i]
i + j < nums.length
You are initially positioned at nums[0].

Return the minimum number of jumps to reach the last position in the array (index nums.length - 1). You may assume there is always a valid answer.

Example 1:

Input: nums = [2,4,1,1,1,1]

Output: 2
Explanation: Jump from index 0 to index 1, then jump from index 1 to the last index.

Example 2:

Input: nums = [2,1,2,1,0]

Output: 2
*/

fn jump(nums: Vec<i32>) -> i32 {
    // greedy approach
    let (mut l, mut r) = (0, 0);
    let mut jump_count = 0;
    while r < nums.len() {
        // all the places we can possibly jump
        r = nums[l] as usize;
        let mut max_idx = 0;
        for idx in (l + 1)..=(l + r) {
            if idx < nums.len() {
                max_idx = std::cmp::max(max_idx, nums[idx] as usize);
            }
        }
        jump_count += 1;
        l = r + max_idx;
    }

    todo!()
}
