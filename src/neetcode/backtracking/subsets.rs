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
    let mut res = Vec::new();

    let mut subset: Vec<i32> = Vec::new();
    fn dfs(i: usize, subset: &mut Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if i >= nums.len() {
            res.push(subset.clone());
            return;
        }

        // include nums[i] in subset
        subset.push(nums[i]);
        dfs(i + 1, subset, nums, res);

        // don't include i in subset
        subset.pop();
        dfs(i + 1, subset, nums, res);
    }

    dfs(0, &mut subset, &nums, &mut res);
    res
}
