package main

func isHappy(n int) bool {
	if n == 1 {
		return true
	}

	history := map[int]int{
		n: 1,
	}

	curr := n
	for curr != 1 {
		sum := 0
		for curr != 0 {
			onesDigit := curr % 10
			sum += onesDigit * onesDigit
			curr /= 10
		}
		curr = sum

		// if we revisit a number even a single time - it's a loop
		history[curr] += 1
		if history[curr] > 1 {
			// likely looping
			return false
		}
	}
	return true
}
