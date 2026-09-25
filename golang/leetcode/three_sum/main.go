package main

import "sort"

// you can pass convert an array into a slice by passing array[:] at the calling boundary?

func threeSum(nums []int) [][]int {
	res := make([][]int, 0)
	sort.Ints(nums)

	for i := 0; i+2 < len(nums); i++ {
		if i > 0 && nums[i] == nums[i-1] {
			// skip duplicates - already tried all possibilities starting from this value
			continue
		}

		j := i + 1
		k := len(nums) - 1

		for j < k {
			total := nums[i] + nums[j] + nums[k]

			if total > 0 {
				k -= 1
			} else if total < 0 {
				j += 1
			} else {
				// 0
				res = append(res, []int{nums[i], nums[j], nums[k]})
				j += 1

				for nums[j] == nums[j-1] && j < k {
					j += 1
				}
			}
		}
	}

	return res

	// O(n^3 solution)

	// hashset of triplets that are sorted
	// triplets := make(map[[3]int]bool)

	// // fix value and two pointers for an O(n^2) solution

	// // checks all combinations of values

	// // three for loops through nums, sort and set triplets to true
	// for i := 0; i < len(nums); i++ {
	// 	for j := i + 1; j < len(nums); j++ {
	// 		for k := j + 1; k < len(nums); k++ {
	// 			if nums[i]+nums[j]+nums[k] == 0 {
	// 				// sort and add to triplets
	// 				t := [3]int{nums[i], nums[j], nums[k]}
	// 				sort.Ints(t[:])
	// 				triplets[t] = true
	// 			}
	// 		}
	// 	}
	// }

	// // build res from triplets
	// res := make([][]int, 0)
	// for t := range triplets {
	// 	res = append(res, []int{t[0], t[1], t[2]})
	// }

	// return res

}
