/*
You are given a network of n directed nodes, labeled from 1 to n. You are also given times, a list of directed edges where times[i] = (ui, vi, ti).

ui is the source node (an integer from 1 to n)
vi is the target node (an integer from 1 to n)
ti is the time it takes for a signal to travel from the source to the target node (an integer greater than or equal to 0).
You are also given an integer k, representing the node that we will send a signal from.

Return the minimum time it takes for all of the n nodes to receive the signal. If it is impossible for all the nodes to receive the signal, return -1 instead.

Example 1:

Input: times = [[1,2,1],[2,3,1],[1,4,4],[3,4,1]], n = 4, k = 1

Output: 3
Example 2:

Input: times = [[1,2,1],[2,3,1]], n = 3, k = 2

Output: -1
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

fn network_delay_time_dfs(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // construct adhacency list repr of graph
    let mut graph = vec![vec![]; (n + 1) as usize];
    for t in times {
        graph[t[0] as usize].push((t[1] as usize, t[2]));
    }

    let mut s = Vec::new();
    // node -> delay
    let mut delays = vec![None; (n + 1) as usize];
    let mut visited = 0;
    s.push((k as usize, 0));
    while let Some((node, delay)) = s.pop() {
        match delays[node] {
            Some(d) => {
                if d <= delay {
                    // already visited this node with a smaller or equal delay
                    continue;
                }
                delays[node] = Some(delay);
            }
            None => {
                delays[node] = Some(delay);
                visited += 1;
            }
        }

        for (edge, weight) in graph[node].iter().copied() {
            s.push((edge, delay + weight));
        }
    }

    if visited != n as usize {
        return -1;
    }

    delays
        .into_iter()
        .skip(1)
        .map(|n| n.unwrap_or_default())
        .max()
        .unwrap_or_default()
}

// bfs impl
// - starting at 1
// - vec impl is gross
// - needs to be a VecDeque if we want pop_front/back
// - when working with a q you usually want to pop front and push back

fn network_delay_time_bfs(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;

    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; (n + 1) as usize];
    for t in times.into_iter() {
        let n1 = t[0] as usize;
        let n2 = t[1] as usize;
        let delay = t[2];

        graph[n1].push((n2, delay));
    }

    let mut q = VecDeque::new();
    q.push_back((k, 0));
    // keep track of min delay for reaching each node
    let mut delays = vec![None; (n + 1) as usize];
    let mut visited = 0;
    while let Some((n, delay)) = q.pop_front() {
        match delays[n] {
            Some(d) => {
                if d <= delay {
                    // already found delay is already min
                    continue;
                }
                delays[n] = Some(delay);
            }
            None => {
                // first time visiting node
                delays[n] = Some(delay);
                visited += 1;
            }
        }

        for (n2, delay2) in graph[n].iter() {
            q.push_back((*n2, delay + delay2));
        }
    }

    if visited != n {
        // didn't visit every node
        return -1;
    }

    // max delay in delays
    delays.into_iter().flatten().max().unwrap()
}

// TODO: dijkstra impl with pqueue (max heap)
// - binary heap in rust is max heap by default
// - lexicographic ordering - compares the first val and only compares second if they are equal
// - needed to use reverse - kinda tired so skipped checking edge cases

fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = n as usize;

    let mut graph = vec![vec![]; (n + 1) as usize];
    for t in times {
        let n1 = t[0] as usize;
        let n2 = t[1] as usize;
        let delay = t[2];
        graph[n1].push((n2, delay));
    }

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, k)));
    let mut delays: Vec<Option<i32>> = vec![None; n + 1];
    let mut visited = 0;
    while let Some(Reverse((delay, node))) = pq.pop() {
        match delays[node] {
            Some(d) => {
                // already visited
                if d < delay {
                    // current spf to node has lower delay
                    continue;
                }

                delays[node] = Some(d);
            }
            None => {
                // first time visiting
                delays[node] = Some(delay);
                visited += 1;
                // end early if we already visited everything - guaranteed to visit min first because pq
                if visited == n {
                    break;
                }
            }
        }
        for (neighbor, neighbor_delay) in graph[node].iter().copied() {
            pq.push(Reverse((delay + neighbor_delay, neighbor)));
        }
    }

    delays.into_iter().flatten().max().unwrap()
}
