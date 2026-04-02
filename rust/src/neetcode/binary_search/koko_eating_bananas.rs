/*
Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.

Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.

Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

Return the minimum integer k such that she can eat all the bananas within h hours.

Example 1:

Input: piles = [3,6,7,11], h = 8
Output: 4
Example 2:

Input: piles = [30,11,23,4,20], h = 5
Output: 30
Example 3:

Input: piles = [30,11,23,4,20], h = 6
Output: 23
*/

// - learned how to use partition_point
// - overlooked that k has a max: size of the largest pile
// - manually implementing binary search here was simpler...

/* fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut k = h / piles.len() as i32;
    let mut prev_k = k;
    loop {
        if attempt_eating(&piles, h, k) {
            println!("first finished attempt at {}", k);

            if k == prev_k {
                // can eat all bananas at 1 banna per hour
                return k;
            }

            // partition_point function
            // true true true false false false
            //
            // returns index of first thing that returns false

            let possible_k: Vec<i32> = ((prev_k + 1)..=k).rev().collect();
            let min_k =
                possible_k[possible_k.partition_point(|&k| attempt_eating(&piles, h, k)) - 1];
            return min_k;
        }
        prev_k = k;
        k *= 2;
    }
}

/// Attempts eating piles at k bananas per hour - returns true if finished eating in time and false otherwise
fn attempt_eating(piles: &Vec<i32>, mut h: i32, k: i32) -> bool {
    let mut finished_in_time = true;
    'outer: for pile in piles {
        let mut pile = *pile;
        while pile > 0 {
            pile -= k;
            if h > 0 {
                h -= 1
            } else {
                // unable to finish eating bananas in time
                finished_in_time = false;
                break 'outer;
            };
            dbg!(pile, k, h);
        }
    }
    finished_in_time
} */

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut k = 1;
    let mut prev_k = k;
    loop {
        if attempt_eating(&piles, h, k) {
            if k == prev_k {
                // 1 is good enough
                return k;
            }

            let mut l = prev_k;
            let mut r = k;
            while l < r {
                let mid = l + (r - l) / 2;
                dbg!(l, r, mid);

                if attempt_eating(&piles, h, mid) {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            return l;
        }
        prev_k = k;
        k *= 2;
    }
}

/// Attempts eating piles at k bananas per hour - returns true if finished eating in time and false otherwise
fn attempt_eating(piles: &Vec<i32>, mut h: i32, k: i32) -> bool {
    let mut finished_in_time = true;
    for &pile in piles {
        // hours for pile
        let mut hours = pile / k;
        if pile % k != 0 {
            hours += 1;
        }

        h -= hours;
        if h < 0 {
            finished_in_time = false;
            break;
        }
    }
    finished_in_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, min_eating_speed(vec![3, 6, 7, 11], 8));
    }
}
