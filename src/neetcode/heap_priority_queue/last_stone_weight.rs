/*
You are given an array of integers stones where stones[i] is the weight of the ith stone.

We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together. Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:

If x == y, both stones are destroyed, and
If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
At the end of the game, there is at most one stone left.

Return the weight of the last remaining stone. If there are no stones left, return 0.

 Example 1:

 Input: stones = [2,7,4,1,8,1]
 Output: 1
 Explanation:
 We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
 we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
 we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
 we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone.
 Example 2:

 Input: stones = [1]
 Output: 1

*/

// Really easy. Used unwrap_or_default which was cool
// Had a logic bug - pop, pop removes both - but if there's only one left cannot remove.
// Had another simple bug - did == instead of >=

use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);
    // start smashing
    // while let (Some(y), Some(x)) = (heap.pop(), heap.pop()) {
    while heap.len() >= 2 {
        let (Some(y), Some(x)) = (heap.pop(), heap.pop()) else {
            panic!("heap len 2 but no two Some")
        };

        // smash
        if x == y {
            // both stones destroyed
        } else {
            // only x is destroyed
            heap.push(y - x);
        }
    }
    heap.pop().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::last_stone_weight;

    #[test]
    fn example_case() {
        let stones = vec![2, 7, 4, 1, 8, 1];
        assert_eq!(last_stone_weight(stones), 1);
    }

    #[test]
    fn single_stone() {
        let stones = vec![5];
        assert_eq!(last_stone_weight(stones), 5);
    }

    #[test]
    fn no_stones() {
        let stones = vec![];
        assert_eq!(last_stone_weight(stones), 0);
    }

    #[test]
    fn all_destroy() {
        let stones = vec![3, 3];
        assert_eq!(last_stone_weight(stones), 0);
    }
}
