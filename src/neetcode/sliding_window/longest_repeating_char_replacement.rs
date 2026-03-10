/*

You are given a string s consisting of only uppercase english characters and an integer k. You can choose up to k characters of the string and replace them with any other uppercase English character.

After performing at most k replacements, return the length of the longest substring which contains only one distinct character.

Example 1:

Input: s = "XYYX", k = 2

Output: 4
Explanation: Either replace the 'X's with 'Y's, or replace the 'Y's with 'X's.

Example 2:

Input: s = "AAABABB", k = 1

Output: 5

*/

// use std::{cmp::max, collections::HashMap};

// initial solution - only looks ahead and never looks behind...
// fn character_replacement(s: String, k: i32) -> i32 {
//     if s.is_empty() {
//         return 0;
//     }

//     let chars = s.as_bytes();
//     let mut max_len = 0;
//     for (l, &c) in chars.iter().enumerate() {
//         let mut replacements = k;
//         let mut r = l + 1;
//         while replacements >= 0 && r < chars.len() {
//             if chars[r] != c {
//                 if replacements == 0 {
//                     break;
//                 }
//                 replacements -= 1;
//             }
//             r += 1;
//         }
//         max_len = max(max_len, r - l);
//     }

//     max_len as i32
// }

// fn character_replacement(s: String, k: i32) -> i32 {
//     if s.is_empty() {
//         return 0;
//     }
//     let chars = s.as_bytes();
//     let mut char_freq = HashMap::with_capacity(26);
//     for l in 0..chars.len() {
//         let mut r = l;
//         let mut max_replacement = 0;
//         let mut max_freq = 0;
//         let mut other_chars = 0;
//         // move pointer to the right
//         while other_chars <= k {
//             // update char freq
//             char_freq
//                 .entry(chars[r])
//                 .and_modify(|e| *e += 1)
//                 .or_insert(1);
//             max_freq = *char_freq.values().max().unwrap();
//             other_chars = (r - l) as i32 + 1 - max_freq;
//             max_replacement = max(max_replacement, (r - l) as i32 + 1);

//             r += 1;
//         }
//     }
//     todo!()
// }

const NUM_ALPHA: usize = 26;

fn character_replacement(s: String, k: i32) -> i32 {
    let get_index = |c: u8| (c - b'A') as usize;
    let s = s.as_bytes();
    // let mut counts = []

    todo!()
}
