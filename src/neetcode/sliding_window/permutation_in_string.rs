/*
Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.

In other words, return true if one of s1's permutations is the substring of s2.

Example 1:

Input: s1 = "ab", s2 = "eidbaooo"
Output: true
Explanation: s2 contains one permutation of s1 ("ba").
Example 2:

Input: s1 = "ab", s2 = "eidboaoo"
Output: false
*/

use std::collections::HashMap;

// naive hashmap impl

fn check_inclusion2(s1: String, s2: String) -> bool {
    // naively
    // - for each char in s2
    // - check if there exists same char in s1
    // - if it does - start sequence
    //   - dont find a match - reset and keep going

    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    // create map of char counts
    // PERF: only lowercase - consider array as optimization
    let mut s1_counts: HashMap<u8, u32> = HashMap::new();
    s1.iter()
        .for_each(|&c| *s1_counts.entry(c).or_default() += 1);

    let mut i = 0;
    while i < s2.len() {
        if let Some(count) = s1_counts.get_mut(&s2[i]) {
            let mut counts_clone = s1_counts.clone();

            // possible permutation sequence found!
            let mut curr = i;
            while !counts_clone.is_empty() && curr < s2.len() {
                if let Some(mut count) = counts_clone.get_mut(&s2[curr]) {
                    *count -= 1;
                    if *count == 0 {
                        counts_clone.remove(&s2[curr]);
                    }
                    curr += 1;
                } else {
                    // couldn't find value
                    break;
                }
            }

            if counts_clone.is_empty() {
                // found valid permutation of s1 managed to remove all chars
                return true;
            }
            // couldn't find permutation - advance i forward
            // i = curr;
        }
        i += 1;
    }
    false
}

// use sliding window and compare counts array for equality

fn check_inclusion(s1: String, s2: String) -> bool {
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());

    if s1.len() > s2.len() {
        return false;
    }

    let mut s1_counts = [0u32; 26];
    let mut window_counts = [0u32; 26];
    // helper - convert char to idx
    let char_idx = |c: u8| (c - b'a') as usize;
    // init counts
    for i in 0..s1.len() {
        s1_counts[char_idx(s1[i])] += 1;
        window_counts[char_idx(s2[i])] += 1;
    }

    if s1_counts == window_counts {
        return true;
    }

    // slide window over and adjust counts as we go
    let mut curr = s1.len();
    while curr < s2.len() {
        // update window
        window_counts[char_idx(s2[curr])] += 1;
        window_counts[char_idx(s2[curr - s1.len()])] -= 1;
        if s1_counts == window_counts {
            return true;
        }
        curr += 1;
    }
    false
}
