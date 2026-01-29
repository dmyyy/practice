/*
Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.

Example 1:

Input: s = "aab"
Output: [["a","a","b"],["aa","b"]]
Example 2:

Input: s = "a"
Output: [["a"]]

*/

fn partition(s: String) -> Vec<Vec<String>> {
    let mut partitions = Vec::new();
    let mut candidates = Vec::new();

    backtrack(&s, &mut partitions, &mut candidates, 0);

    partitions
}

pub fn partition2(s: String) -> Vec<Vec<String>> {
    let mut partitions = Vec::new();
    let mut candidates = Vec::new();

    backtrack(&s, &mut partitions, &mut candidates, 0);

    partitions
}

pub fn backtrack(
    s: &String,
    partitions: &mut Vec<Vec<String>>,
    candidates: &mut Vec<String>,
    start: usize,
) {
    if start == s.len() {
        partitions.push(candidates.clone());
        return;
    }
    for i in start..s.len() {
        let candidate = &s[start..i + 1];
        if is_palindrome(candidate) {
            candidates.push(candidate.to_owned());
            backtrack(s, partitions, candidates, i + 1);
            candidates.pop();
        }
    }
}

pub fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
