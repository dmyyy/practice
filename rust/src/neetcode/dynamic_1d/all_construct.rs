// TODO:

// passing around ownership of vec lead to issues - keeping a &mut is better

use std::collections::HashMap;

fn all_construct(target: &'static str, words: Vec<&'static str>) -> Vec<Vec<String>> {
    // recursive backtracking - brute force explore the search space
    fn all_construct_helper(
        target: &'static str,
        words: &Vec<&'static str>,
        curr: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
    ) {
        if target.is_empty() {
            // found
            res.push(curr.clone());
            return;
        }

        for word in words {
            if word.len() > target.len() {
                continue;
            }

            if **word == target[0..word.len()] {
                // found a match - recurse
                curr.push(String::from(*word));
                all_construct_helper(&target[word.len()..], &words, curr, res);
                curr.pop();
            }
        }
    }

    let mut curr = Vec::new();
    let mut res = Vec::new();
    all_construct_helper(target, &words, &mut curr, &mut res);
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        dbg!(all_construct(
            "purple",
            vec!["purp", "p", "ur", "le", "purpl"]
        ));
    }
}
