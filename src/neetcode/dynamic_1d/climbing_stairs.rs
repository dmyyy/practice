/*
You are given an integer n representing the number of steps to reach the top of a staircase. You can climb with either 1 or 2 steps at a time.

Return the number of distinct ways to climb to the top of the staircase.

Example 1:

Input: n = 2

Output: 2
Explanation:

1 + 1 = 2
2 = 2
Example 2:

Input: n = 3

Output: 3
*/

use std::collections::HashMap;

fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    assert!(n >= 3);

    // step -> permutations possible
    let mut map = HashMap::<i32, i32>::new();
    map.insert(n - 1, 1);
    map.insert(n - 2, 2);

    let mut curr = n - 3;
    while map.get(&0).is_none() {
        let one = *map.get(&(curr + 1)).unwrap();
        let two = *map.get(&(curr + 2)).unwrap();

        map.insert(curr, one + two);

        curr -= 1;
    }

    *map.get(&0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}
