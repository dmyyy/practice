package main

// expand around center (uses constant space)

func countSubstrings(s string) int {
	n := len(s)
	count := 0

	expand := func(l, r int) {
		for l >= 0 && r < n && s[l] == s[r] {
			count++
			l--
			r++
		}
	}

	for i := range n {
		// odd length starting at i
		expand(i, i)
		// even length
		expand(i, i+1)
	}

	return count
}

// dp solution

/*
func countSubstrings(s string) int {
	n := len(s)
	if n == 0 {
		return 0
	}

	dp := make([][]bool, n)
	for i := range dp {
		dp[i] = make([]bool, n)
	}
	count := 0
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			if s[i] == s[j] && (j-i < 2 || dp[i+1][j-1]) {
				dp[i][j] = true
				count++
			}
		}
	}
	return count
}
*/

/*
func countSubstrings(s string) int {
	var isPalindrome func(l, r int) bool
	isPalindrome = func(l, r int) bool {
		for l <= r {
			if s[l] != s[r] {
				return false
			}
			l++
			r--
		}
		return true
	}

	count := len(s)
	for i := 0; i < len(s); i++ {
		for j := i + 1; j < len(s); j++ {
			if isPalindrome(i, j) {
				count += 1
			}
		}
	}

	return count
}
*/

// first impl - hashing each string is slow, not sure if caching everything is a good idea

/*
func countSubstrings(s string) int {
	// time complexity: O(n^3)

	// check all possible permutations
	count := 0
	cache := make(map[string]bool)
	for i := range len(s) {
		for j := i + 1; j <= len(s); j++ {
			cacheVal, ok := cache[s[i:j]]
			if cacheVal {
				// already seen this string + it's a palindrome
				count += 1
			} else if ok {
				// already seen string and not palindrome
				continue
			} else if isPalindrome(s[i:j]) {
				count += 1
				cache[s[i:j]] = true
			} else {
				cache[s[i:j]] = false
			}
		}
	}

	return count
}

func isPalindrome(s string) bool {
	l, r := 0, len(s)-1
	for l <= r {
		if s[l] != s[r] {
			return false
		}
		l++
		r--
	}
	return true
}
*/
