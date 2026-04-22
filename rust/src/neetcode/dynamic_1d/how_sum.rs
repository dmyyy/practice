use std::collections::HashMap;

// passing around &mut[T] never helpful if you want to iterate over
// using raw indices is the most flexible approach
// &[T] can be helpful

fn how_sum(target: u32, nums: Vec<u32>) -> Vec<u32> {
    fn how_sum_helper(curr: u32, res: Vec<u32>, target: u32, nums: &[u32]) -> Option<Vec<u32>> {
        if curr > target {
            // overshot - not the answer
            return None;
        }
        if curr == target {
            return Some(res);
        } else {
            for &num in nums {
                let mut new_res = res.clone();
                new_res.push(num);
                if let Some(res) = how_sum_helper(curr + num, new_res, target, &nums[1..]) {
                    return Some(res);
                }
            }
        }
        None
    }

    // return how_sum or panic
    how_sum_helper(0, vec![], target, nums.as_slice()).unwrap()
}

// framing in terms of remainder (until target) instead of curr can be helpful? that's what the video does as well...
// don't really understand memo

fn how_sum2(target: u32, nums: Vec<u32>) -> Option<Vec<u32>> {
    fn how_sum_helper(curr: u32, res: Vec<u32>, target: u32, nums: &Vec<u32>) -> Option<Vec<u32>> {
        if curr > target {
            // overshot - not the answer
            return None;
        }
        if curr == target {
            return Some(res);
        } else {
            for &num in nums {
                let mut new_res = res.clone();
                new_res.push(num);
                if let Some(res) = how_sum_helper(curr + num, new_res, target, &nums) {
                    return Some(res);
                }
            }
        }
        None
    }

    how_sum_helper(0, vec![], target, &nums)
}

// same as how_sum2 but now we memoize
// - build up res on the way down and return it if it's a correct result and throw it away otherwise
// - hard to memo - want to do partial work as we recurse back up...
// -
//
// - building up result as we go back up tree is usually better* (at least in this case)
//
// time complexity: O(n^m)
// - at worst recurses target times
// space complexity: O(m) on the stack

fn how_sum3(target: u32, nums: Vec<u32>) -> Option<Vec<u32>> {
    fn how_sum_helper(rem: u32, nums: &Vec<u32>) -> Option<Vec<u32>> {
        if rem == 0 {
            return Some(vec![]);
        } else {
            for &num in nums {
                if num > rem {
                    continue;
                }

                if let Some(mut res) = how_sum_helper(rem - num, nums) {
                    res.push(num);
                    return Some(res);
                }
            }
        }
        None
    }

    how_sum_helper(target, &nums)
}

// memo for how_sum3
//
// time complexity: O(n*m*m)
// - at worst target distinct sub-problems
// space complexity: O(n*m + m^2)

fn how_sum4(target: u32, nums: Vec<u32>) -> Option<Vec<u32>> {
    fn how_sum_helper(
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
                if let Some(cached_res) = memo.get(&next_rem) {
                    return cached_res.clone();
                }

                if let Some(mut res) = how_sum_helper(next_rem, nums, memo) {
                    res.push(num);
                    memo.insert(next_rem, Some(res.clone()));
                    return Some(res);
                }
            }
        }

        memo.insert(rem, None);
        None
    }

    let mut memo = HashMap::default();
    how_sum_helper(target, &nums, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(how_sum(7, vec![5, 3, 4, 7]), vec![3, 4]);
        // assert_eq!(how_sum2(0, vec![]), Some(vec![]));
        // assert_eq!(how_sum2(7, vec![2, 4]), None);
        // assert_eq!(how_sum2(8, vec![2, 3, 5]), Some(vec![2, 2, 2, 2]));

        assert_eq!(how_sum3(7, vec![5, 3, 4, 7]), Some(vec![4, 3]));
        assert_eq!(how_sum3(0, vec![]), Some(vec![]));
        assert_eq!(how_sum3(7, vec![2, 4]), None);
        assert_eq!(how_sum3(8, vec![2, 3, 5]), Some(vec![2, 2, 2, 2]));
        // assert_eq!(how_sum3(300, vec![7, 14]), None);
        assert_eq!(how_sum4(300, vec![7, 14]), None);
    }
}
