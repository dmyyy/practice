package main

import "fmt"

func removeDuplicates(nums []int) int {

	p := new(int)
	fmt.Println(p)

	k := 1
	deleted := 0
	for i := 1; i < len(nums); i++ {
		prev := nums[i-1]
		if nums[i] == prev {
			nums = append(nums[:i], nums[i+1:]...)
			deleted += 1
			i -= 1
		} else {
			k += 1
		}
	}
	return k
}
