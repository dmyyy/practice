/*
You are given an array of non-overlapping intervals intervals where intervals[i] = [start_i, end_i] represents the start and the end time of the ith interval. intervals is initially sorted in ascending order by start_i.

You are given another interval newInterval = [start, end].

Insert newInterval into intervals such that intervals is still sorted in ascending order by start_i and also intervals still does not have any overlapping intervals. You may merge the overlapping intervals if needed.

Return intervals after adding newInterval.

Note: Intervals are non-overlapping if they have no common point. For example, [1,2] and [3,4] are non-overlapping, but [1,2] and [2,3] are overlapping.

Example 1:

Input: intervals = [[1,3],[4,6]], newInterval = [2,5]

Output: [[1,6]]
Example 2:

Input: intervals = [[1,2],[3,5],[9,10]], newInterval = [6,7]

Output: [[1,2],[3,5],[6,7],[9,10]]
*/

use std::cmp::{max, min};

fn insert(intervals: Vec<[u32; 2]>, mut new_interval: [u32; 2]) -> Vec<[u32; 2]> {
    let mut res = Vec::new();

    for (idx, [start, end]) in intervals.iter().enumerate() {
        if new_interval[1] < *start {
            res.push(new_interval);
            res.append(&mut intervals[idx..].to_vec());
            return res;
        } else if new_interval[0] > *end {
            res.push([*start, *end]);
        } else {
            new_interval = [min(new_interval[0], *start), max(new_interval[1], *end)]
        }
    }
    res.push(new_interval);

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(insert(vec![[1, 3], [4, 6]], [2, 5]), vec![[1, 6]]);
        assert_eq!(
            insert(vec![[1, 2], [3, 5], [9, 10]], [6, 7]),
            vec![[1, 2], [3, 5], [6, 7], [9, 10]]
        );
    }
}
