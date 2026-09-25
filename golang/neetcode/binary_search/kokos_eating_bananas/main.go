package main

import (
	"fmt"
	"math"
	"slices"
)

/*
You are given an integer array piles where piles[i] is the number of bananas in the ith pile. You are also given an integer h, which represents the number of hours you have to eat all the bananas.

You may decide your bananas-per-hour eating rate of k. Each hour, you may choose a pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, you may finish eating the pile but you can not eat from another pile in the same hour.

Return the minimum integer k such that you can eat all the bananas within h hours.

Example 1:

Input: piles = [1,4,3,2], h = 9

Output: 2
Explanation: With an eating rate of 2, you can eat the bananas in 6 hours. With an eating rate of 1, you would need 10 hours to eat all the bananas (which exceeds h=9), thus the minimum eating rate is 2.

Example 2:

Input: piles = [25,10,23,4], h = 4

Output: 25
*/

func minEatingSpeed(piles []int, h int) int {
	slices.Sort(piles)

	lowerBound := piles[0]
	upperBound := piles[len(piles)-1]
	possibleEatingSpeeds := make([]int, upperBound-lowerBound+1)

	// populate possibleEatingSpeeds
	speed := lowerBound
	for i := range possibleEatingSpeeds {
		possibleEatingSpeeds[i] = speed
		speed += 1
	}

	minSpeed := math.MaxInt32
	l, r := 0, len(possibleEatingSpeeds)-1
	for l <= r {
		m := l + (r-l)/2

		// attempt eating piles of bananas and see how long it takes
		timeTaken := eatingTime(possibleEatingSpeeds[m], slices.Clone(piles))
		if timeTaken > h {
			r = m - 1
		} else if timeTaken < h {
			if possibleEatingSpeeds[m] < minSpeed {
				minSpeed = possibleEatingSpeeds[m]
			}
			l = m + 1
		} else {
			return possibleEatingSpeeds[m]
		}
	}
	return minSpeed
}

func eatingTime(eatingSpeed int, piles []int) int {
	hours_taken := 0
	for i := range piles {
		for piles[i] > 0 {
			piles[i] -= eatingSpeed
			hours_taken += 1
		}
	}
	return hours_taken
}

func main() {
	// Input: piles = [1,4,3,2], h = 9
	piles := []int{1, 4, 3, 2}
	fmt.Println(minEatingSpeed(piles, 9))
}
