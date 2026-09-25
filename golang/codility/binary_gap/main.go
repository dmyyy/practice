package main

import "math/bits"

func binaryGap(n int) int {
	largestBinaryGap := 0
	currBinaryGap := 0
	startedGap := false
	for i := 0; i < bits.UintSize; i++ {
		if n&1 == 1 {
			if startedGap {
				// gap ends here
				startedGap = false
				if currBinaryGap > largestBinaryGap {
					largestBinaryGap = currBinaryGap
				}
				currBinaryGap = 0
			} else {
				// gap starts here
				startedGap = true
			}
		} else {
			// 0
			currBinaryGap += 1
		}
		n >>= 1
	}

	return largestBinaryGap
}
