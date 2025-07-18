/*
Given an array nums of unique integers, return all possible subsets of nums.

The solution set must not contain duplicate subsets. You may return the solution in any order.

Example 1:

Input: nums = [1,2,3]

Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
Example 2:

Input: nums = [7]

Output: [[],[7]]
Constraints:

1 <= nums.length <= 10
-10 <= nums[i] <= 10
*/

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    for subset_size in 0..nums.len() {
        subsets_helper(subset_size, vec![], &nums, &mut res);
    }

    todo!();
}

// oof this one kinda hard for me even though it shouldn't imo - will need to read the answer later

fn subsets_helper(subset_size: usize, subset: Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if subset.len() == subset_size {
        res.push(subset);
    } else {
        if 
    }
}
