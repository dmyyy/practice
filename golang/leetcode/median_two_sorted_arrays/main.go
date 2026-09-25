package main

import "sort"

// len(arr) / 2 - gives middle if len is odd, and right side of middle if len is even

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	// naive solution
	// - merge nums1 and nums2
	// - return middle number (or average of two middle numbers if there is an even number of numbers)

	// time: O((n+m)log(n+m))
	// space: O(n+m)

	/*
		merged := append(nums1, nums2...)
		sort.Ints(merged)

		if len(merged) % 2 != 0 {
			// odd - return merged[len(merged)/2]
			return float64(merged[len(merged)/2])
		} else {
			// even - average two middle values
			mid := len(merged) / 2
			return float64((merged[mid] + merged[mid-1])) / 2
		}
	*/

	// faster merge in O(n+m) with a two pointer approach since original slices are already sorted

	/*
		merged := make([]int, 0, len(nums1)+len(nums2))
		l, r := 0, 0
		for l+r < len(merged) {
			if l < len(nums1) && r < len(nums2) {
				if nums1[l] < nums2[r] {
					merged[l+r] = nums1[l]
					l += 1
				} else {
					merged[l+r] = nums2[r]
					r += 1
				}
			} else if l < len(nums1) {
				merged[l+r] = nums1[l]
				l += 1
			} else if r < len(nums2) {
				merged[l+r] = nums2[r]
				r += 1
			}
		}
	*/

	// binary search w/ partitioning

	// - looking for kth smallest element twice

	// time: O(log(n+m))

	if len(nums1) > len(nums2) {
		nums1, nums2 = nums2, nums1
	}

	m, n := len(nums1), len(nums2)
	low, high := 0, m




	return 0.
}
