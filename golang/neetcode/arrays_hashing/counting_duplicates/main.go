package main

import "fmt"

/*
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Example 1:

Input: nums = [1,2,3,1]
Output: true
Example 2:

Input: nums = [1,2,3,4]
Output: false
Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]
Output: true
*/

func containsDuplicate(nums []int) bool {
	counts := make(map[int]int)
	for _, num := range nums {
		counts[num] += 1
		count := counts[num]
		if count >= 2 {
			return true
		}
	}
	return false
}

func main() {
	testSlice := []int{1, 2, 3, 1}
	fmt.Println(containsDuplicate(testSlice))
}
