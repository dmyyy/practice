/*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]

Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Explanation:

There is no string in strs that can be rearranged to form "bat".
The strings "nat" and "tan" are anagrams as they can be rearranged to form each other.
The strings "ate", "eat", and "tea" are anagrams as they can be rearranged to form each other.
Example 2:

Input: strs = [""]

Output: [[""]]

Example 3:

Input: strs = ["a"]

Output: [["a"]]
*/


use indexmap::IndexMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: IndexMap<Vec<char>, Vec<String>> = IndexMap::new();
    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();

        map.entry(chars)
            .and_modify(|strs| strs.push(s.clone()))
            .or_insert(vec![s]);
    }

    let mut res: Vec<Vec<String>> = Vec::new();
    for (_, v) in map {
        let mut rev_v = v.clone();
        rev_v.reverse();
        res.push(rev_v);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // use a map so annoying to get the right order, but the code works
        let input_slices: Vec<&str> = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let input_strings: Vec<String> = input_slices.into_iter().map(|s| s.to_string()).collect();
        let output: Vec<Vec<&str>> =
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        assert_eq!(output, group_anagrams(input_strings));
    }
}
