package main

func getConcatenation(nums []int) []int {
	ans := make([]int, 2*len(nums))
	n := len(nums)
	for i, num := range nums {
		ans[i] = num
		ans[i+n] = num
	}
	return ans
}
