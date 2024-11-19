/*
Given an array of meeting time interval objects consisting of start and end times [[start_1,end_1],[start_2,end_2],...] (start_i < end_i), determine if a person could add all meetings to their schedule without any conflicts.

Example 1:

Input: intervals = [(0,30),(5,10),(15,20)]

Output: false
Explanation:

(0,30) and (5,10) will conflict
(0,30) and (15,20) will conflict
Example 2:

Input: intervals = [(5,8),(9,15)]

Output: true
*/

use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd)]
struct Interval {
    start: u32,
    end: u32,
}

impl Interval {
    pub fn new(start: u32, end: u32) -> Self {
        if start > end {
            panic!("tried to create interval where start > end");
        }
        Self { start, end }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.end < other.start {
            return Ordering::Less;
        } else if self.start > other.end {
            return Ordering::Greater;
        } else {
           return Ordering::Equal;
        }
    }
}

fn can_attend_meetings(mut intervals: Vec<Interval>) -> bool {
    // does any interval overlap with any other interval?

    // brute force o n^2 solution compare every possible interval w/ each other
    // sort all intervals -> do a single pass

    intervals.sort();

    for i in 0..(intervals.len() - 1) {
        if intervals {

        } 
    for interval in intervals {

    } 

    todo!()
}
