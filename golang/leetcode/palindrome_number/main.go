package main

func isPalindrome(x int) bool {
	if x < 0 {
		// negative numbers cannot be palindromes
		return false
	} else if x < 10 {
		// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 are already palindromes
		return true
	}

	// copy of x so we don't change it
	x2 := x
	revX := 0
	for x2 != 0 {
		// make space in revX for adding another number
		revX *= 10

		digit := x2 % 10
		revX += digit
		x2 /= 10
	}

	return x == revX
}
