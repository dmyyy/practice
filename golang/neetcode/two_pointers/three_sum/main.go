package main

import "fmt"

/*

Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] where nums[i] + nums[j] + nums[k] == 0, and the indices i, j and k are all distinct.

The output should not contain any duplicate triplets. You may return the output and the triplets in any order.

Example 1:

Input: nums = [-1,0,1,2,-1,-4]

Output: [[-1,-1,2],[-1,0,1]]
Explanation:
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
The distinct triplets are [-1,0,1] and [-1,-1,2].

Example 2:

Input: nums = [0,1,1]

Output: []
Explanation: The only possible triplet does not sum up to 0.

Example 3:

Input: nums = [0,0,0]

Output: [[0,0,0]]
Explanation: The only possible triplet sums up to 0.

*/

type Triplet struct {
	
	a int
	b int
	c int
}

// brute force
// TODO: order of triplet shouldn't matter though..
func threeSum(nums []int) [][]int {
	visited := make(map[
	res := make([][]int, 0)
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			for k := j + 1; k < len(nums); k++ {
				if nums[i] +nums[j] +nums[k] == 0{
					// verify a similar triplet hasn't been added
					// add triplet to results
						
				}
			}
		}
	}
}

// func threeSum(nums []int) [][]int {
// 	if len(nums) < 3 {
// 		// can't make a triplet
// 		return nil
// 	}
//
// 	res := make([][]int, 0)
// 	isFinished := false
// 	first := 0
// 	second := 1
// 	third := 2
//
// 	// used to dedup
// 	triplets := make(map[[3]int]bool)
// 	for !isFinished {
// 		if nums[first]+nums[second]+nums[third] == 0 && !triplets[[3]int{nums[first], nums[second], nums[third]}] {
// 			// add to res
// 			res = append(res, []int{nums[first], nums[second], nums[third]})
// 			triplets[[3]int{nums[first], nums[second], nums[third]}] = true
// 		}
//
// 		third += 1
// 		if third == len(nums) {
// 			second += 1
// 			if second == len(nums)-1 {
// 				first += 1
// 				if first == len(nums)-2 {
// 					// done
// 					return res
// 				}
// 				second = first + 1
// 			}
// 			third = second + 1
// 		}
// 	}
// 	panic("unreachable")
// }

func main() {
	nums := []int{-1, 0, 1, 2, -1, -4}
	fmt.Println(threeSum(nums))
}
