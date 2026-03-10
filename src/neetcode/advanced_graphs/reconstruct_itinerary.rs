/*
You are given a list of airline tickets where tickets[i] = [fromi, toi] represent the departure and the arrival airports of one flight. Reconstruct the itinerary in order and return it.

All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK". If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.

For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.

Example 1:

Input: tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
Output: ["JFK","MUC","LHR","SFO","SJC"]
Example 2:


Input: tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"] but it is larger in lexical order.
*/

use std::collections::{HashMap, HashSet};

// letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
// string.entry(ch).and_modify(|counter| *coutner += 1).or_insert(1);

// aight i give up for now - some hiemholzer algorithm or whatever + this exceeds timelimit zzz

fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    // adj list
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut tickets = tickets;
    tickets.sort();
    for t in &tickets {
        graph.entry(t[0].clone()).or_default().push(t[1].clone());
    }

    let mut res = vec!["JFK".to_owned()];
    let target_len = tickets.len() + 1;

    fn dfs(
        from: &str,
        res: &mut Vec<String>,
        graph: &mut HashMap<String, Vec<String>>,
        target_len: usize,
    ) -> bool {
        if res.len() == target_len {
            // found result!
            return true;
        }
        let dests = match graph.get(from) {
            Some(d) => d.clone(),
            None => return false,
        };
        for i in 0..dests.len() {
            let v = dests[i].clone();
            graph.get_mut(from).unwrap().remove(i);
            res.push(v.clone());
            if dfs(&v, res, graph, target_len) {
                return true;
            }
            graph.get_mut(from).unwrap().insert(i, v);
            res.pop();
        }
        false
    }

    dfs("JFK", &mut res, &mut graph, target_len);

    res
}
