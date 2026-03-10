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

// - core idea behind greedy: do the best thing we can do at every turn
// - wtf it worked first try - took me a little bit though..

fn jump(nums: Vec<i32>) -> i32 {
    // to avoid annoying casting
    let nums: Vec<usize> = nums.into_iter().map(|i| i as usize).collect();

    let mut num_jumps = 0;
    let mut curr_idx = 0;
    // nums guaranteed to be of at least length 1
    while curr_idx != nums.len() - 1 {
        let mut next_idx = curr_idx;
        let mut max_next_idx = 0;
        for maybe_next_idx in (curr_idx + 1)..=(curr_idx + nums[curr_idx]) {
            if maybe_next_idx == nums.len() - 1 {
                // can already exit on the current jump!
                return num_jumps + 1;
            }
            // choose next_idx that maximizes the next furthest jump
            let maybe_next_jump_len = nums[maybe_next_idx];
            let maybe_next_max_idx = maybe_next_idx + maybe_next_jump_len;
            if maybe_next_max_idx > max_next_idx {
                // new max
                max_next_idx = maybe_next_max_idx;
                next_idx = maybe_next_idx;
            }
        }
        num_jumps += 1;
        curr_idx = next_idx;
    }
    return num_jumps;
}
