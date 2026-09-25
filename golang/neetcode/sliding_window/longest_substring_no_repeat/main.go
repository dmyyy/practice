package main

import (
	"fmt"
	"strings"
)

/*
Given a string s, find the length of the longest substring without duplicate characters.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "zxyzxyz"

Output: 3
Explanation: The string "xyz" is the longest without duplicate characters.

Example 2:

Input: s = "xxxx"

Output: 1
*/

// what I needed
// - thing we were maximizing
// - current window size
// - window start point
//
// - if map doesn't have a repeat then keep going
// - otherwise

func longestSubstringNoRepeat(s string) int {
	longestSubstring := -1
	currentSubstring := 0
	windowStart := 0
	charCounts := make(map[rune]int)
	for _, r := range s {
		if charCounts[r] == 0 {
			charCounts[r] += 1
			currentSubstring += 1
			if currentSubstring > longestSubstring {
				longestSubstring = currentSubstring
			}
		} else {
			for _, r := range s[windowStart : strings.IndexRune(s, r)+1] {
				charCounts[r] -= 1
				currentSubstring -= 1
			}
			windowStart = strings.IndexRune(s, r) + 1
		}
	}
	return longestSubstring
}

func main() {
	s := string("zxyzxyz")
	fmt.Println(longestSubstringNoRepeat(s))
}
