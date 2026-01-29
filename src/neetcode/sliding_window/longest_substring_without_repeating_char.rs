/*

Given a string s, find the length of the longest substring without duplicate characters.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "zxyzxyz"

Output: 3
Explanation: The string "xyz" is the longest without duplicate characters.

Example 2:

Input: s = "xxxx"

Output: 1

*/

use std::cmp::max;
use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let chars = s.as_bytes();
    let mut unique_chars = HashSet::new();

    let mut start = 0;
    let mut max_len = 0;

    for end in 0..chars.len() {
        while unique_chars.contains(&chars[end]) {
            unique_chars.remove(&chars[start]);
            start += 1;
        }
        unique_chars.insert(chars[end]);

        // window is from start..=end
        max_len = max(max_len, end - start + 1);
    }

    max_len as i32
}
