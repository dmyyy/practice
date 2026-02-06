/*
Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

Example 1:

Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
Example 2:

Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.
Example 3:

Input: intervals = [[4,7],[1,4]]
Output: [[1,7]]
Explanation: Intervals [1,4] and [4,7] are considered overlapping.
*/

use std::cmp::max;

use crate::neetcode::intervals;

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: need to sort the intervals first?

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut merged_intervals = Vec::new();
    let mut i = 0;
    while i < intervals.len() {
        let start = intervals[i][0];
        let mut end = intervals[i][1];
        let mut j = i + 1;
        while j < intervals.len() {
            if end >= intervals[j][0] {
                // overlapping interval - merge
                end = max(end, intervals[j][1]);
                j += 1;
            } else {
                // not overlapping - break
                break;
            }
        }
        merged_intervals.push(vec![start, end]);
        i = j;
    }

    merged_intervals
}
