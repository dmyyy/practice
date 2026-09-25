package main

import (
	"container/heap"
)

// in go [] do not dereference - *slice, must deref slice first before using
// MUST USE HEAP methods for it to work correctly

type MinHeap []int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(int))
}
func (h *MinHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

type MaxHeap []int

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[j] < h[i] }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x any) {
	*h = append(*h, x.(int))
}
func (h *MaxHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

type MedianFinder struct {
	botHalf *MaxHeap
	topHalf *MinHeap
}

func Constructor() MedianFinder {
	bh := &MaxHeap{}
	heap.Init(bh)
	th := &MinHeap{}
	heap.Init(th)

	return MedianFinder{
		botHalf: bh,
		topHalf: th,
	}
}

func (this *MedianFinder) AddNum(num int) {
	// first value in stream
	if this.botHalf.Len() == 0 {
		heap.Push(this.botHalf, num)
		return
	}

	// insert
	if num > (*this.botHalf)[0] {
		// top half
		heap.Push(this.topHalf, num)
	} else {
		heap.Push(this.botHalf, num)
	}

	// rebalance
	if this.botHalf.Len() > this.topHalf.Len()+1 {
		heap.Push(this.topHalf, heap.Pop(this.botHalf))
	} else if this.topHalf.Len() > this.botHalf.Len() {
		heap.Push(this.botHalf, heap.Pop(this.topHalf))
	}
}

func (this *MedianFinder) FindMedian() float64 {
	total := this.botHalf.Len() + this.topHalf.Len()
	if total%2 == 0 {
		// even - take average of bot and top
		return float64((*this.botHalf)[0]+(*this.topHalf)[0]) / 2.0
	} else {
		// odd - take max of bot half
		return float64((*this.botHalf)[0])
	}
}
