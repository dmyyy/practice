package main

import (
	"fmt"
	"maps"
)

/*
You are given two strings s1 and s2.

Return true if s2 contains a permutation of s1, or false otherwise. That means if a permutation of s1 exists as a substring of s2, then return true.

Both strings only contain lowercase letters.

Example 1:

Input: s1 = "abc", s2 = "lecabee"

Output: true
Explanation: The substring "cab" is a permutation of "abc" and is present in "lecabee".

Example 2:

Input: s1 = "abc", s2 = "lecaabee"

Output: false
*/

func checkInclusion(s1 string, s2 string) bool {
	// add all from window start to window end to a set
	// subarray is going to be of size s1

	// index of first character in window
	windowStart := 0
	// index of last character in window
	windowEnd := len(s1) - 1
	presentCharacters := make(map[byte]int)
	for _, b := range []byte(s2[windowStart : windowEnd+1]) {
		presentCharacters[b] += 1
	}

	s1Counts := make(map[byte]int)
	for _, b := range []byte(s1) {
		s1Counts[b] += 1
	}

	// the count map for s1 needs to match presentCharacters
	for windowEnd < len(s2)-1 {
		fmt.Println("present charcters: ", presentCharacters)
		fmt.Println("s1 counts", s1Counts)

		// cannot compare two maps for equality if they just set things to 0
		if maps.Equal(presentCharacters, s1Counts) {
			// permutation exists
			return true
		}

		presentCharacters[s2[windowStart]] -= 1
		if presentCharacters[s2[windowStart]] == 0 {
			delete(presentCharacters, s2[windowStart])
		}

		windowStart += 1
		windowEnd += 1
		presentCharacters[s2[windowEnd]] += 1
	}
	return false
}

func main() {
	fmt.Println(checkInclusion("abc", "lecabee"))
}
