package main

import (
	"fmt"
	"slices"
)

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

type Interval struct {
	start uint
	end   uint
}

func insert(intervals []Interval, newInterval Interval) []Interval {
	// iterate intervals
	// if we find an interval overlapping with start merge that into new interval

	if len(intervals) == 0 {
		intervals = append(intervals, newInterval)
		return intervals
	}

	// can cleanly merge (just insert, and don't need to modify the array
	// overlapping - need to merge multiple

	res := make([]Interval, len(intervals))
	copy(res, intervals)

	overlapping := false
	overlap_start := -1
	overlap_end := -1
	insertion_point := -1
	for i, interval := range intervals {

		if interval.start <= newInterval.end || interval.end >= newInterval.start {
			overlapping = true
			if overlap_start == -1 {
				overlap_start = i
			}
			overlap_end = i
			newInterval = Interval{min(interval.start, newInterval.start), max(interval.end, newInterval.end)}
		}

		if !overlapping && newInterval.end < interval.start {
			// found insertion point
			insertion_point = i
		}
	}

	fmt.Printf("overlapping: %v\n", overlapping)
	fmt.Printf("overlap_start: %v\n", overlap_start)
	fmt.Printf("overlap_end: %v\n", overlap_end)
	fmt.Printf("newInterval: %v\n", newInterval)
	fmt.Printf("insertion_point: %v\n", insertion_point)

	if overlapping {
		// need to remove all indices in overlapping_intervals
		// need to do +1 because non-inclusive
		res = slices.Replace(res, overlap_start, overlap_end+1, newInterval)
		return res
	}

	if insertion_point == -1 {
		// insert at the end
		res = slices.Insert(res, len(intervals), newInterval)
		return res
	} else {
		// clean insertion point at some index
		res = slices.Insert(res, insertion_point, newInterval)
	}

	return res
}

func main() {
	intervals := []Interval{{1, 3}, {4, 6}}
	newInterval := Interval{2, 5}
	fmt.Println(insert(intervals, newInterval))
}
