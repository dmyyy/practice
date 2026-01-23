/*

Given an array of integers numbers that is sorted in non-decreasing order.

Return the indices (1-indexed) of two numbers, [index1, index2], such that they add up to a given target number target and index1 < index2. Note that index1 and index2 cannot be equal, therefore you may not use the same element twice.

There will always be exactly one valid solution.

Your solution must use
O
(
1
)
O(1) additional space.

Example 1:

Input: numbers = [1,2,3,4], target = 3

Output: [1,2]
Explanation:
The sum of 1 and 2 is 3. Since we are assuming a 1-indexed array, index1 = 1, index2 = 2. We return [1, 2].
*/

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // brute force n^2 solution
    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().enumerate().skip(i + 1) {
            let diff = target - n1 - n2;
            if diff == 0 {
                // found a solution
                return vec![i as i32 + 1, j as i32 + 1];
            } else if diff < 0 {
                // input is sorted - if target - n1 - n2 < 0 we can break early
                break;
            }
        }
    }
    // no possible solution..
    unreachable!("guaranteed to have exactly one solution!")
}
