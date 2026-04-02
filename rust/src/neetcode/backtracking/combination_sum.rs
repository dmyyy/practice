/*
You are given an array of distinct integers nums and a target integer target. Your task is to return a list of all unique combinations of nums where the chosen numbers sum to target.

The same number may be chosen from nums an unlimited number of times. Two combinations are the same if the frequency of each of the chosen numbers is the same, otherwise they are different.

You may return the combinations in any order and the order of the numbers in each combination can be in any order.

Example 1:

Input:
nums = [2,5,6,9]
target = 9

Output: [[2,2,5],[9]]
Explanation:
2 + 2 + 5 = 9. We use 2 twice, and 5 once.
9 = 9. We use 9 once.

Example 2:

Input:
nums = [3,4,5]
target = 16

Output: [[3,3,3,3,4],[3,3,5,5],[4,4,4,4],[3,4,4,5]]
Example 3:

Input:
nums = [3]
target = 5

Output: []
*/

use std::collections::HashSet;

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn sum_recursive(
        candidates: &Vec<i32>,
        target: i32,
        combination: &mut Vec<i32>,
        sum: i32,
        combinations: &mut HashSet<Vec<i32>>,
    ) {
        if sum > target {
            return;
        } else if sum == target {
            let mut combination = combination.clone();
            combination.sort();
            combinations.insert(combination);
        } else {
            for &c in candidates {
                combination.push(c);
                sum_recursive(candidates, target, combination, sum + c, combinations);
                combination.pop();
            }
        }
    }

    let mut combinations = HashSet::new();
    let mut combination = Vec::new();
    sum_recursive(&candidates, target, &mut combination, 0, &mut combinations);

    // de-dup answers based on frequency of numbers
    // count map for each?
    // sort each?
    // sort before trying

    combinations.into_iter().collect::<Vec<Vec<i32>>>()
}
