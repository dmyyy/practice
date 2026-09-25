package main

// if I want dp - need to have some kind of state to dp on. Original recursive backtracking
// soln didn't have that - it BLAH

// TODO: try rewrite this w/ int stuff

func numDecodings(s string) int {
	count := 0
	var decodingsHelper func(s string)
	decodingsHelper = func(s string) {
		if len(s) == 0 {
			count++
			return
		}

		digit := int(s[0] - '0')
		if digit == 0 {
			return
		} else if digit == 1 && len(s) > 1 {
			decodingsHelper(s[2:])
		} else if digit == 2 && len(s) > 1 {
			if int(s[1]-'0') < 7 {
				decodingsHelper(s[2:])
			}
		}
		decodingsHelper(s[1:])
	}
	decodingsHelper(s)
	return count
}

// recursive backtracking O(2^n solution)

/* func numDecodings(s string) int {
	count := 0
	var decodingsHelper func(s string)
	decodingsHelper = func(s string) {
		if len(s) == 0 {
			count++
			return
		}

		digit := int(s[0] - '0')
		if digit == 0 {
			return
		} else if digit == 1 && len(s) > 1 {
			decodingsHelper(s[2:])
		} else if digit == 2 && len(s) > 1 {
			if int(s[1]-'0') < 7 {
				decodingsHelper(s[2:])
			}
		}
		decodingsHelper(s[1:])
	}
	decodingsHelper(s)
	return count
} */
