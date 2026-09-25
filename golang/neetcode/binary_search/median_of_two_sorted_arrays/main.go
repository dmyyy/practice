package main

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	nums1Len, nums2Len := len(nums1), len(nums2)
	numsLen := nums1Len + nums2Len
	if numsLen%2 != 0 {
		// odd
		k := numsLen/2 + 1
		return float64(findKthSmallest(k, nums1, nums2))
	} else {
		// even
		k1 := numsLen / 2
		k2 := numsLen/2 + 1
		k1Num := findKthSmallest(k1, nums1, nums2)
		k2Num := findKthSmallest(k2, nums1, nums2)
		return float64(k1Num+k2Num) / 2.
	}
}

func findKthSmallest(k int, nums1 []int, nums2 []int) int {
	// slices are sorted
	// use binary search to eliminate k/2 elements each iteration
	// - check k/2 elements in nums1/nums2
	// - eliminate the elements in array with smaller value (guaranteed to not be median)

	if len(nums1) == 0 {
		return nums2[k-1]
	}
	if len(nums2) == 0 {
		return nums1[k-1]
	}

	if k == 1 {
		if nums1[0] < nums2[0] {
			return nums2[0]
		}
		return nums1[0]
	}

	i := min(k/2, len(nums1))
	j := min(k/2, len(nums2))

	if nums1[i-1] <= nums2[j-1] {
		return findKthSmallest(k-i, nums1[i:], nums2)
	}
	return findKthSmallest(k-j, nums1, nums2[j:])
}
