package main

import "slices"

// must use the return value freom slices Insert

func plusOne(digits []int) []int {
	carry := false
	for i := len(digits) - 1; i >= 0; i-- {
		if digits[i] == 9 {
			digits[i] = 0
			carry = true
		} else {
			digits[i] += 1
			carry = false
			break
		}
	}
	if carry {
		digits = slices.Insert(digits, 0, 1)
	}
	return digits
}
