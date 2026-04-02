/*
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.



Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false


Constraints:

1 <= s.length, t.length <= 5 * 104
s and t consist of lowercase English letters.
*/

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *s_counts.entry(c).or_default() += 1;
    }

    let mut t_counts: HashMap<char, usize> = HashMap::new();
    for c in t.chars() {
        *t_counts.entry(c).or_default() += 1;
    }

    s_counts == t_counts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
