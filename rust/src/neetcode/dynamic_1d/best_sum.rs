use std::collections::HashMap;

// returns shortest combination of numbers

// attempt at changing memoized how_sum4 into best_sum
// failed because we build up array as we go back up in recursion - don't fully explore the tree because of return

fn best_sum(target: u32, nums: Vec<u32>) -> Option<Vec<u32>> {
    fn best_sum_helper(
        rem: u32,
        nums: &Vec<u32>,
        memo: &mut HashMap<u32, Option<Vec<u32>>>,
    ) -> Option<Vec<u32>> {
        if rem == 0 {
            return Some(vec![]);
        } else {
            for &num in nums {
                if num > rem {
                    continue;
                }

                let next_rem = rem - num;
                if let Some(mut res) = best_sum_helper(next_rem, nums, memo) {
                    res.push(num);

                    dbg!(next_rem, &res, &memo.get_mut(&next_rem));

                    // check size of vec in memo - if this one is smaller then we update
                    if let Some(Some(cached_res)) = memo.get_mut(&next_rem)
                        && res.len() < cached_res.len()
                    {
                        // found a shorter solution - replace cached_res with it
                        let _ = std::mem::replace(cached_res, res);
                        return Some(cached_res.clone());
                    } else {
                        memo.insert(next_rem, Some(res.clone()));
                        return Some(res);
                    }
                }
            }
        }

        memo.insert(rem, None);
        None
    }

    let mut memo = HashMap::default();
    best_sum_helper(target, &nums, &mut memo)
}

// either
// - build bottom up (start building once we reach base case and build up as we go up recursive tree)
// - build as we go down the tree

// bottom up wasn't the issue - issue was returning inside the for loop
// correct thing to do is to continue exploring the tree outside of for loop and blah

// works but too slow bcuz not memoized

fn best_sum2(target: u32, nums: Vec<u32>) -> Option<Vec<u32>> {
    fn best_sum_helper(rem: u32, nums: &Vec<u32>) -> Option<Vec<u32>> {
        if rem == 0 {
            return Some(vec![]);
        } else {
            let mut shortest_res: Option<Vec<u32>> = None;
            for &num in nums {
                if num > rem {
                    continue;
                }

                if let Some(mut res) = best_sum_helper(rem - num, nums) {
                    res.push(num);

                    if let Some(shortest_res) = &mut shortest_res
                        && shortest_res.len() > res.len()
                    {
                        *shortest_res = res;
                    } else if shortest_res.is_none() {
                        shortest_res = Some(res);
                    }

                    dbg!(rem, &shortest_res);
                }
            }
            return shortest_res;
        }
    }

    best_sum_helper(target, &nums)
}

// memoize best_sum2
//
// trick was to memo outside of the for loop

fn best_sum3(target: u32, nums: Vec<u32>) -> Option<Vec<u32>> {
    fn best_sum_helper(
        rem: u32,
        nums: &Vec<u32>,
        memo: &mut HashMap<u32, Option<Vec<u32>>>,
    ) -> Option<Vec<u32>> {
        if rem == 0 {
            return Some(vec![]);
        } else {
            if let Some(cached_res) = memo.get(&(rem)) {
                // already have a shortest_res calculated for this rem
                return cached_res.clone();
            }

            // shortest_res for rem
            let mut shortest_res: Option<Vec<u32>> = None;
            for &num in nums {
                if num > rem {
                    continue;
                }

                if let Some(mut res) = best_sum_helper(rem - num, nums, memo) {
                    res.push(num);

                    if let Some(shortest_res) = &mut shortest_res
                        && shortest_res.len() > res.len()
                    {
                        *shortest_res = res;
                    } else if shortest_res.is_none() {
                        shortest_res = Some(res);
                    }

                    dbg!(rem, &shortest_res);
                }
            }
            memo.insert(rem, shortest_res.clone());
            return shortest_res;
        }
    }

    let mut memo = HashMap::default();
    best_sum_helper(target, &nums, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(best_sum(7, vec![5, 3, 4, 7]), vec![3, 4]);
        // assert_eq!(how_sum2(0, vec![]), Some(vec![]));
        // assert_eq!(how_sum2(7, vec![2, 4]), None);
        // assert_eq!(how_sum2(8, vec![2, 3, 5]), Some(vec![2, 2, 2, 2]));

        // assert_eq!(how_sum3(7, vec![5, 3, 4, 7]), Some(vec![4, 3]));
        // assert_eq!(how_sum3(0, vec![]), Some(vec![]));
        // assert_eq!(how_sum3(7, vec![2, 4]), None);
        // assert_eq!(how_sum3(8, vec![2, 3, 5]), Some(vec![2, 2, 2, 2]));
        // assert_eq!(how_sum3(300, vec![7, 14]), None);
        // assert_eq!(best_sum2(8, vec![1, 4, 5]), Some(vec![4, 4]));
        assert_eq!(best_sum3(8, vec![1, 4, 5]), Some(vec![4, 4]));
        assert_eq!(best_sum3(100, vec![1, 5, 25]), Some(vec![25, 25, 25, 25]));
    }
}
