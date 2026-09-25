package main

import (
	"fmt"
	"math"
)

/*

You are given an integer array prices where prices[i] is the price of NeetCoin on the ith day.

You may choose a single day to buy one NeetCoin and choose a different day in the future to sell it.

Return the maximum profit you can achieve. You may choose to not make any transactions, in which case the profit would be 0.

Example 1:

Input: prices = [10,1,5,6,7,1]

Output: 6
Explanation: Buy prices[1] and sell prices[4], profit = 7 - 1 = 6.

Example 2:

Input: prices = [10,8,7,5,2]

Output: 0
Explanation: No profitable transactions can be made, thus the max profit is 0.

*/

// sliding window - max value of first + last value in a sliding window

func maxProfit(prices []int) int {
	maxProfit := -1
	for i, selling_value := range prices {
		// determine if buying price or selling price

		// - consider i the selling value
		// - we need to buy at the lowest point previously seen
		// - iterate 0 to i and find the lowest point
		// - that is the maximum sum for that value

		// find min buying point before curr selling value
		min_buying_price := math.MaxInt
		for j := 0; j < i; j++ {
			if prices[j] < min_buying_price {
				min_buying_price = prices[j]
			}
		}

		// update max profit
		if selling_value-min_buying_price > maxProfit {
			maxProfit = selling_value - min_buying_price
		}
	}
	return maxProfit
}

func main() {
	prices := []int{10, 1, 5, 6, 7, 1}
	fmt.Println(maxProfit(prices))
}
