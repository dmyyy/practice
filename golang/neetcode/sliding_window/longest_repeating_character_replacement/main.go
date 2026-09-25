package main

import "fmt"

/*
You are given a string s consisting of only uppercase english characters and an integer k. You can choose up to k characters of the string and replace them with any other uppercase English character.

After performing at most k replacements, return the length of the longest substring which contains only one distinct character.

Example 1:

Input: s = "XYYX", k = 2

Output: 4
Explanation: Either replace the 'X's with 'Y's, or replace the 'Y's with 'X's.

Example 2:

Input: s = "AAABABB", k = 1

Output: 5
*/
func characterReplacement(s string, k int) int {
	// find out what k replacements need to be done
	//
	// find the longest substring
	//

	// analyze the string and find out which characters we want to replace

	// sliding window
	// - first character is potential start of longest substring
	// - longest substring is at least letter itself + k other letters that we change to be the same
	// - can replace w/ any english letter but replacement should already be in the string
	// longest substring at each starting character
	longestSubstringLength := -1
	for windowStart, r := range s {
		currSubstringLength := 1
		currSubstring := string(r)
		fmt.Println("start ", currSubstring)
		replacements := k
		for _, r2 := range s[windowStart+1:] {
			fmt.Println("start len", currSubstringLength)
			if r == r2 {
				currSubstringLength += 1
				currSubstring += string(r2)

				fmt.Println("substring length", currSubstringLength)

				if longestSubstringLength < currSubstringLength {
					longestSubstringLength = currSubstringLength
				}

				continue
			}

			fmt.Println("mid len", currSubstringLength)

			if replacements > 0 {
				replacements -= 1
				currSubstringLength += 1
				currSubstring += string(r)
				if longestSubstringLength < currSubstringLength {
					longestSubstringLength = currSubstringLength
				}
				fmt.Println("replaced ", currSubstring)
				fmt.Println("length ", currSubstringLength)
				// update longest substring
			} else {
				// trying to add a different character
				fmt.Println(currSubstring)
				fmt.Println(r2)
				break
			}

		}
	}

	return longestSubstringLength
}

func main() {
	s := "XYYX"
	fmt.Println(characterReplacement(s, 2))
}
