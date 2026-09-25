package main

import (
	"fmt"
)

/*
Given an integer array nums, return an array output where output[i] is the product of all the elements of nums except nums[i].

Each product is guaranteed to fit in a 32-bit integer.

Follow-up: Could you solve it in O(n) time without using the division operation?

Example 1:

Input: nums = [1,2,4,6]

Output: [48,24,12,8]
Example 2:

Input: nums = [-1,0,1,2,3]

Output: [0,-6,0,0,0]
*/

func product_except_self(nums []int) []int {
	// fp[i] = fp[0]*..*fp[i]
	forward_products := make([]int, len(nums))
	forward_products[0] = nums[0]
	for i := 1; i < len(nums); i++ {
		prev := forward_products[i-1]
		forward_products[i] = nums[i] * prev
	}

	fmt.Println(forward_products)

	backward_products := make([]int, len(nums))
	backward_products[len(nums)-1] = nums[len(nums)-1]
	for i := len(nums) - 2; i >= 0; i-- {
		prev := backward_products[i+1]
		backward_products[i] = nums[i] * prev
	}

	fmt.Println(backward_products)

	products := make([]int, len(nums))
	for i := range nums {
		var forward_product int
		if i == 0 {
			forward_product = 1
		} else {
			forward_product = forward_products[i-1]
		}

		var backward_product int
		if i == len(nums)-1 {
			backward_product = 1
		} else {
			backward_product = backward_products[i+1]
		}

		products[i] = forward_product * backward_product
	}

	fmt.Println(products)

	return products
}

func main() {
	nums := []int{1, 2, 4, 6}
	fmt.Println(product_except_self(nums))
}
