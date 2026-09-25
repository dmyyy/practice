package main

import "sort"

// type Seq struct {
// 	seq []int,
// 	count int
// }

func longestConsecutive(nums []int) int {
	// cant sort - nlogn

	if len(nums) == 0 {
		return 0
	}

	sort.Ints(nums)
	maxSeqLen := 1
	currSeqLen := 1
	for i := 1; i < len(nums); i++ {
		prev := nums[i-1]

		if prev == nums[i] {
			// duplicate - skip
			continue
		}
		if prev == nums[i]-1 {
			currSeqLen += 1
		} else {
			// finished seq
			if currSeqLen > maxSeqLen {
				maxSeqLen = currSeqLen
			}

			currSeqLen = 1
		}
	}

	if currSeqLen > maxSeqLen {
		maxSeqLen = currSeqLen
	}

	return maxSeqLen
}
