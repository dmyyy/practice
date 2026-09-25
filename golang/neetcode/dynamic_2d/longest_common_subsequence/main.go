package main

func longestCommonSubsequence(text1 string, text2 string) int {
	// in the smaller string
	// for each char
	// - try to build the longest common subsequence possible

	// text1 is always the smaller one
	if len(text1) > len(text2) {
		text1, text2 = text2, text1
	}

	maxSubsequenceLen := 0
	findLCS(text1, text2, &maxSubsequenceLen)

	return 0
}

// try to fit text1 into text2 trying starting at every character in text2
func findLCS(text1 string, text2 string, maxSubsequenceLen *int) {
	if len(text1) == 0 {
		return
	}
	// issue: need to continue text2 from the right char each time...
	for i := 0; i < len(text2); i++ {
		if text2[i] == text1[0] {
			// possible start point - see how big of a subsequence we can build starting from i
			subsequenceLen := 1
			j := 1

			for j < len(text1) && i < len(text2) {
				if text1[i] == text2[j] {
					// same char - extend subsequence
					subsequenceLen++
					j += 1
					i += 1
					break
				}
			}

			// update max subsequence after
			if subsequenceLen > *maxSubsequenceLen {
				*maxSubsequenceLen = subsequenceLen
			}
		}
	}
	findLCS(text1[1:], text2, maxSubsequenceLen)
}
