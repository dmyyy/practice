package main

import (
	"fmt"
	"sort"
)

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

// overlapping intervals
// - merge or consolidate ranges
// - schedule or find conflicts (meeting rooms)
// - find gaps or missing intervals

// no overlap - cannot merge
// a overlaps b - merged = [min(starta, startb), max(enda, endb)]

type Interval struct {
	start uint
	end   uint
}

func canAttendMeetings(intervals []Interval) bool {
	// iterate through all intervals
	// see if any intervals are overlapping
	// are intervals ordered?

	if len(intervals) <= 1 {
		return true
	}

	// sort by start time
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i].start < intervals[j].start
	})

	for i := 1; i < len(intervals); i++ {
		// if curr start < prev end - overlap
		if intervals[i].start < intervals[i-1].end {
			return false
		}
	}
	return true
}

func main() {
	intervals := []Interval{{0, 30}, {5, 10}, {15, 20}}
	fmt.Println(canAttendMeetings(intervals))
}
