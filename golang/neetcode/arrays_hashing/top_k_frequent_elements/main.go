package main

import (
	"container/heap"
	"fmt"
)

/*
Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

Example 1:

Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
Example 2:

Input: nums = [1], k = 1
Output: [1]
*
*/

type Node struct {
	n     int
	count int
}

type IntHeap []Node

func (h IntHeap) Len() int {
	return len(h)
}

// min heap, will always return smallest value first
// val with highest count is "smallest"
func (h IntHeap) Less(i, j int) bool {
	return h[i].count > h[j].count
}

func (h IntHeap) Swap(i, j int) {
	// swapped using multi value assignment
	h[i], h[j] = h[j], h[i]
}

// for heap interface - DO NOT USE
func (h *IntHeap) Push(x any) {
	*h = append(*h, x.(Node))
}

func (h *IntHeap) Pop() any {
	n := len(*h)
	x := (*h)[n-1]
	// IMPORTANT: go slice ranges are inclusive of the last element
	// similar to rusts ..= syntax
	*h = (*h)[0 : n-1]

	// old := *h
	// n := len(old)
	// x := old[n-1]
	// *h = old[0 : n-1]

	// probably heap impl is subtly wrong...
	return x
}

// need to make sure to ask if I can use docs during interview
// don't be smart and just copy paaste heap impl if allowed

func topKFrequent(nums []int, k int) []int {
	h := &IntHeap{}
	heap.Init(h)

	counts := make(map[int]int)
	for _, n := range nums {
		counts[n] += 1
	}

	for n, count := range counts {
		fmt.Printf("push node: %v", n)
		heap.Push(h, Node{n, count})
	}

	fmt.Printf("counts: %v", counts)

	res := make([]int, 0)
	// pull k items from heap
	for range k {
		node, _ := heap.Pop(h).(Node)
		fmt.Printf("pop node: %v", node)
		res = append(res, node.n)
	}
	return res
}

func main() {
	nums := []int{1, 1, 1, 2, 2, 2, 3, 3, 3}
	fmt.Println(topKFrequent(nums, 3))
}
