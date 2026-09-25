package main

import (
	"fmt"
	"strings"
)

/*
Design an algorithm to encode a list of strings to a single string. The encoded string is then decoded back to the original list of strings.

Please implement encode and decode

Example 1:

Input: ["neet","code","love","you"]

Output:["neet","code","love","you"]
Example 2:

Input: ["we","say",":","yes"]

Output: ["we","say",":","yes"]
*/

func encode(strs []string) string {
	return strings.Join(strs, "|")
}

func decode(s string) []string {
	return strings.Split(s, "|")
}

func main() {
	input := []string{"neet", "code", "love", "you"}
	encoded_input := encode(input)

	fmt.Println(encoded_input)
	fmt.Println(decode(encoded_input))
}

