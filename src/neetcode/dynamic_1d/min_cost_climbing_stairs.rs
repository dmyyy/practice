/*
You are given an array of integers cost where cost[i] is the cost of taking a step from the ith floor of a staircase. After paying the cost, you can step to either the (i + 1)th floor or the (i + 2)th floor.

You may choose to start at the index 0 or the index 1 floor.

Return the minimum cost to reach the top of the staircase, i.e. just past the last index in cost.

Example 1:

Input: cost = [1,2,3]

Output: 2
Explanation: We can start at index = 1 and pay the cost of cost[1] = 2 and take two steps to reach the top. The total cost is 2.

Example 2:

Input: cost = [1,2,1,2,1,1,1]

Output: 4
*/

use std::{cmp::min, collections::HashMap};

fn min_cost_climbing_stairs(cost: Vec<usize>) -> usize {
    if cost.len() == 2 {
        return min(cost[0], cost[1]);
    }

    // step -> min_cost to get to that step
    let mut running_cost = HashMap::<usize, usize>::new();
    running_cost.insert(0, cost[0]);
    running_cost.insert(1, cost[1]);

    let mut curr = 2;
    while !running_cost.contains_key(&(cost.len() - 1)) {
        let one = *running_cost.get(&(curr - 1)).unwrap();
        let two = *running_cost.get(&(curr - 2)).unwrap();

        running_cost.insert(curr, cost[curr] + min(one, two));
        curr += 1;
    }

    let last = *running_cost.get(&(cost.len() - 1)).unwrap();
    let second_to_last = *running_cost.get(&(cost.len() - 2)).unwrap();
    min(last, second_to_last)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(min_cost_climbing_stairs(vec![1, 2, 3]), 2);
        assert_eq!(min_cost_climbing_stairs(vec![1, 2, 1, 2, 1, 1, 1]), 4);
    }
}
