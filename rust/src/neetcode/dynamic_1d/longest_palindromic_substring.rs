/*
Given a string s, return the longest palindromic substring in s.

Example 1:

Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.
Example 2:

Input: s = "cbbd"
Output: "bb"

Constraints:

1 <= s.length <= 1000
s consist of only digits and English letters.
*/

fn longest_palindrome(s: String) -> String {
    let chars = s.into_bytes();

    fn find_longest_palindrome(p: String, chars: &[u8], longest_palindrome: &mut &str) {
        for i in 0..chars.len() {
            if is_palindrome(p) && p.len() > longest_palindrome.len() {
                *longest_palindrome = p;
            }

            find_longest_palindrome(p + chars[0], &chars[1..], longest_palindrome);
            find_longest_palindrome("", &chars[1..], longest_palindrome);
        }
    }

    let mut res = String::from("");
    find_longest_palindrome("".to_owned(), chars.as_slice(), &mut res);

    todo!()
}

fn is_palindrome(p: &str) -> bool {
    p.chars().eq(p.chars().rev())
}
