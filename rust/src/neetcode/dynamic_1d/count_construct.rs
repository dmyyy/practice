// brute-force count_construct

use std::collections::HashMap;

fn count_construct(target: &'static str, words: Vec<&'static str>) -> u32 {
    // recursive backtracking - brute force explore the search space
    fn count_construct_helper(target: &'static str, words: &Vec<&'static str>, count: &mut u32) {
        if target.is_empty() {
            // found a combination!
            *count += 1;
        } else {
            for word in words {
                if word.len() > target.len() {
                    continue;
                }

                if **word == target[0..word.len()] {
                    // found a match - recurse
                    count_construct_helper(&target[word.len()..], &words, count);
                }
            }
        }
    }

    let mut count = 0;
    count_construct_helper(target, &words, &mut count);
    count
}

// memoize original solution
//
// returning things is easier to memo overall... (add cache before return statements and check memo at top)

fn count_construct2(target: &'static str, words: Vec<&'static str>) -> u32 {
    // recursive backtracking - brute force explore the search space
    fn count_construct_helper(
        target: &'static str,
        words: &Vec<&'static str>,
        count: &mut u32,
        memo: &mut HashMap<&str, u32>,
    ) {
        if target.is_empty() {
            // found a combination!
            *count += 1;
            return;
        }

        if let Some(&cached) = memo.get(target) {
            *count += cached;
            return;
        }

        let start_count = *count;
        for word in words {
            if word.len() > target.len() {
                continue;
            }

            if **word == target[0..word.len()] {
                // found a match - recurse
                count_construct_helper(&target[word.len()..], &words, count, memo);
            }
        }
        // cache amount of new matches found in this iteration
        memo.insert(target, *count - start_count);
    }

    let mut memo = HashMap::default();
    let mut count = 0;
    count_construct_helper(target, &words, &mut count, &mut memo);
    count
}

// rewritten to return 1 and memo total count at each level of the tree

// n: target len
// m: num words
//
// worst case:
// n - height of tree
// m - branching factor
//
// time complexity: O(m*n^n)
// space complexity: O(n)

fn count_construct3(target: &'static str, words: Vec<&'static str>) -> u32 {
    // recursive backtracking - brute force explore the search space
    fn count_construct_helper(
        target: &'static str,
        words: &Vec<&'static str>,
        memo: &mut HashMap<&str, u32>,
    ) -> u32 {
        if let Some(&cached) = memo.get(target) {
            return cached;
        }

        if target.is_empty() {
            // found a combination!
            return 1;
        }

        let mut count = 0;
        for word in words {
            if word.len() > target.len() {
                continue;
            }

            if **word == target[0..word.len()] {
                // found a match - recurse
                count += count_construct_helper(&target[word.len()..], &words, memo);
            }
        }
        // cache amount of new matches found in this iteration
        memo.insert(target, count);
        return count;
    }

    let mut memo = HashMap::default();
    count_construct_helper(target, &words, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            count_construct("abcdef", vec!["a", "ab", "abc", "def", "cdef"]),
            2
        );
        assert_eq!(
            count_construct("purple", vec!["purp", "p", "ur", "le", "purpl"]),
            2
        );

        assert_eq!(
            count_construct2("abcdef", vec!["a", "ab", "abc", "def", "cdef"]),
            2
        );
        assert_eq!(
            count_construct2("purple", vec!["purp", "p", "ur", "le", "purpl"]),
            2
        );
        // only runs fast enough bcuz memoized
        assert_eq!(
            count_construct2(
                "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
                vec!["e", "ee", "eee", "eeee", "eeeee"]
            ),
            0
        );

        assert_eq!(
            count_construct3("abcdef", vec!["a", "ab", "abc", "def", "cdef"]),
            2
        );
        assert_eq!(
            count_construct3("purple", vec!["purp", "p", "ur", "le", "purpl"]),
            2
        );
        // only runs fast enough bcuz memoized
        assert_eq!(
            count_construct3(
                "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
                vec!["e", "ee", "eee", "eeee", "eeeee"]
            ),
            0
        );
    }
}
