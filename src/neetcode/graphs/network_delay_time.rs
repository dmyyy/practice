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
Constraints:

1 <= k <= n <= 100
1 <= times.length <= 1000
*/

// Not done - initial instinct was to try brute force with BFS. Needed Dijkstra's

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let adjacency_list = {
        let mut res = vec![vec![]; n + 1];
        for time in times {
            let u = time[0] as usize;
            let v = time[1] as usize;
            let w = time[2];
            res[u].push((v, w));
        }
        res
    };

    // dijkstra
    let mut distances = vec![std::i32::MAX; n + 1];
    distances[k] = 0;
    let mut visited = vec![false; n + 1];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), k));
    while let Some((Reverse(dist), u)) = pq.pop() {
        if visited[u] {
            continue;
        }
    }

    // // do a bfs
    // // iterate through all times, and find all the edges that start at node k
    // // done if we see all n nodes

    // let mut times_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    // for t in times.iter() {
    //     times_map.entry(t[0]).or_default().push((t[1], t[2]));
    // }
    // // WARN: careful this is 0 indexed but nodes start at 1
    // let mut visited_nodes = vec![false; n as usize];
    // let mut q = VecDeque::new();
    // q.extend(times_map.get(&k).unwrap().clone());

    // while let Some((u, delay)) = q.pop_back() {
    //     if !visited_nodes[(u - 1) as usize] {
    //         // node being visited for the first time.
    //     }
    // }

    // bfs?
    // for

    // want to do a bfs starting at the kth node
    todo!()
}
