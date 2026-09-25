package main

/*
You are given an array of CPU tasks tasks, where tasks[i] is an uppercase english character from A to Z. You are also given an integer n.

Each CPU cycle allows the completion of a single task, and tasks may be completed in any order.

The only constraint is that identical tasks must be separated by at least n CPU cycles, to cooldown the CPU.

Return the minimum number of CPU cycles required to complete all tasks.

Example 1:

Input: tasks = ["X","X","Y","Y"], n = 2

Output: 5
Explanation: A possible sequence is: X -> Y -> idle -> X -> Y.

Example 2:

Input: tasks = ["A","A","A","B","C"], n = 3

Output: 9
Explanation: A possible sequence is: A -> B -> C -> Idle -> A -> Idle -> Idle -> Idle -> A.
*/

import (
	"container/heap"
	"fmt"
)

// An Item is something we manage in a priority queue.
type Item struct {
	value    string // The value of the item; arbitrary.
	priority int    // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	// index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	// pq[i].index = i
	// pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	// n := len(*pq)
	item := x.(*Item)
	// item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil // don't stop the GC from reclaiming the item eventually
	// item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// update modifies the priority and value of an Item in the queue.
// func (pq *PriorityQueue) update(item *Item, value string, priority int) {
// 	item.value = value
// 	item.priority = priority
// 	heap.Fix(pq, item.index)
// }

func leastInterval(tasks []string, n int) int {
	// get counts of each task
	counts := make(map[string]int)
	for _, t := range tasks {
		counts[t] += 1
	}

	cooldowns := make(map[string]int)
	for s := range counts {
		cooldowns[s] = 0
	}

	// pq based on most occurring
	pq := make(PriorityQueue, len(counts))
	idx := 0
	for s, count := range counts {
		x := &Item{s, count}
		pq[idx] = x
		idx += 1
	}
	heap.Init(&pq)

	cpu_cycles := 0
	for len(pq) != 0 {
		// keep pulling from heap, choose the item that
		// - has a count greater than 0
		//		- if count is 0 don'd add back to the heap
		// - not on cooldown based on cd map
		//
		// - each iteration - update cd map
		//

		// just reduce priority of node everytime you remove...
		removed_tasks := make([]*Item, 0)
		task_ran := false
		for !task_ran && pq.Len() >= 1 {
			task := heap.Pop(&pq).(*Item)
			if task.priority > 0 {
				if cooldowns[task.value] == 0 {
					// run task
					task_ran = true
					cooldowns[task.value] = n + 1
					task.priority -= 1
				}

				if task.priority > 0 {
					removed_tasks = append(removed_tasks, task)
				}
			}
		}

		// add all tasks in removed tasks back
		for _, task := range removed_tasks {
			heap.Push(&pq, task)
		}

		cpu_cycles += 1
		// decrement cooldowns of all tasks
		// put removed tasks back on heap if their cd isn't 0
		for task := range cooldowns {
			if cooldowns[task] > 0 {
				cooldowns[task] -= 1
			}
		}
	}
	return cpu_cycles
}

func main() {
	tasks := []string{"X", "X", "Y", "Y"}
	fmt.Println(leastInterval(tasks, 2))
}
