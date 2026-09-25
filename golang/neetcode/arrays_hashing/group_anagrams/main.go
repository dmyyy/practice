package main

import (
	"fmt"
	"maps"
)

/*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]

Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Explanation:

There is no string in strs that can be rearranged to form "bat".
The strings "nat" and "tan" are anagrams as they can be rearranged to form each other.
The strings "ate", "eat", and "tea" are anagrams as they can be rearranged to form each other.
Example 2:

Input: strs = [""]

Output: [[""]]

Example 3:

Input: strs = ["a"]

Output: [["a"]]
*/

func groupAnagrams(strs []string) [][]string {
	anagram_maps := make([]map[rune]int, 0, len(strs))
	res := make([][]string, 0, len(strs))

	// two strings are anagrams if they have equal rune_counts maps
	for _, s := range strs {
		rune_counts := make(map[rune]int, len(s))
		for _, rune := range s {
			rune_counts[rune] += 1
		}

		// see if there are any other maps that match
		hasAnagram := false
		for i, m := range anagram_maps {
			if maps.Equal(rune_counts, m) {
				hasAnagram = true
				res[i] = append(res[i], s)
			}
		}
		if !hasAnagram {
			anagram_maps = append(anagram_maps, rune_counts)
			res = append(res, []string{s})
		}
	}
	return res
}

func main() {
	strs := []string{"eat", "tea", "tan", "ate", "nat", "bat"}
	fmt.Println(groupAnagrams(strs))
}
