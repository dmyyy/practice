package main

import (
	"fmt"
)

func merge(nums1 []int, m int, nums2 []int, n int) {
	if n == 0 {
		// no work to do
		return
	}

	// kinda weird - we NEED to override the zeros
	// can use copy to shift over the elements - not sure how useful this is buuuut...

	i := 0
	j := 0
	for i < m+j && j < n {
		if nums1[i] > nums2[j] {
			// shift over to make space for nums2[j]
			copy(nums1[i+1:m+j+1], nums1[i:m+j])
			nums1[i] = nums2[j]
			j += 1
			i += 1
		} else if nums1[i] <= nums2[j] {
			i += 1
		}
		fmt.Printf("i: %d, j: %d, nums1: %v\n", i, j, nums1)
	}
	for j < n {
		nums1[m+j] = nums2[j]
		j += 1
		fmt.Printf("i: %d, j: %d, nums1: %v\n", i, j, nums1)
	}
}
