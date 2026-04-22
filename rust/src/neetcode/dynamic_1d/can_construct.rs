use std::collections::HashMap;

// m: target_len
// n: word_bank_len
//
// m is the height of the tree
// n is worst-case branching factor
//
// in general: branching-factor^tree-height
//
// time complexity: O(n^m*m)
// space complexity: O(m)

// simple memo (before trying anything too smart)
// - put memo at the very top
// - memo right before recursive returns
//
// time complexity: O(n*m^2)
// space complexity: O(m)

fn can_construct(target: &'static str, word_bank: Vec<&'static str>) -> bool {
    fn can_construct_helper(
        target: &'static str,
        word_bank: &Vec<&'static str>,
        memo: &mut HashMap<&'static str, bool>,
    ) -> bool {
        if let Some(cached_res) = memo.get(target) {
            return *cached_res;
        }
        if target.is_empty() {
            return true;
        } else {
            for word in word_bank {
                if word.len() > target.len() {
                    continue;
                }

                // this == comparison is O(m) where m is length of the target string in the worst case
                if **word == target[0..word.len()] {
                    let res = can_construct_helper(&target[word.len()..], &word_bank, memo);
                    if res {
                        memo.insert(&target, res);
                        return true;
                    }
                }
            }
        }
        memo.insert(&target, false);
        false
    }

    // memo false
    let mut memo = HashMap::default();
    can_construct_helper(target, &word_bank, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(can_construct("", vec!["bar", "baz", "foo"]));
        assert!(can_construct("foobar", vec!["bar", "baz", "foo"]));
        assert!(!can_construct("foobarqux", vec!["bar", "baz", "foo"]));
        assert!(can_construct(
            "abcdef",
            vec!["ab", "abc", "cd", "def", "abcd"]
        ));
    }
}
