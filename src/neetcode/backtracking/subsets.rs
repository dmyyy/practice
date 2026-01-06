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

// new uncommitted lines here!!

// TODO: do this

// fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
//     todo!()
// }

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut result = Vec::new();

    for mask in 0..(1 << n) {
        let mut subset = Vec::new();
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                subset.push(nums[i]);
            }
        }
        result.push(subset);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input = vec![];
        let mut output = subsets(input);
        let mut expected = vec![vec![]];
        output.sort();
        expected.sort();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_single_element() {
        let input = vec![1];
        let mut output = subsets(input);
        let mut expected = vec![vec![], vec![1]];
        output.sort();
        expected.sort();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_two_elements() {
        let input = vec![1, 2];
        let mut output = subsets(input);
        let mut expected = vec![vec![], vec![1], vec![2], vec![1, 2]];
        output.sort();
        expected.sort();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_three_elements() {
        let input = vec![1, 2, 3];
        let mut output = subsets(input);
        let mut expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        output.sort();
        expected.sort();
        assert_eq!(output, expected);
    }
}
