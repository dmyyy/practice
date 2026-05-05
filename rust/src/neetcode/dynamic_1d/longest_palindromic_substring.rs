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

// need to remember eq function on an iterator - can use it to compare iterators
// if I ever want to do my 'static str shenanigans in a leetcode problem - JUST LEAK THE STRING
// str::from_utf8 to convert &[u8] to &str

// brute force solution w/o memo

pub fn longest_palindrome(s: String) -> String {
    fn longest_palindrome_helper(chars: &'static [u8], longest: &mut &'static str) {
        if chars.len() <= longest.len() {
            // cannot create a longer palindrome from remaining string
            return;
        }
        for i in 1..=chars.len() {
            if i > longest.len() && is_palindrome(&chars[0..i]) {
                *longest = str::from_utf8(&chars[0..i]).unwrap();
            }
        }
        longest_palindrome_helper(&chars[1..], longest);
    }

    let chars = s.leak().as_bytes();
    let longest = &mut str::from_utf8(&chars[0..1]).unwrap();
    longest_palindrome_helper(chars, longest);
    longest.to_owned()
}

fn is_palindrome(chars: &[u8]) -> bool {
    chars.iter().eq(chars.iter().rev())
}

// TODO: do with memo

pub fn longest_palindrome2(s: String) -> String {
    fn longest_palindrome_helper(chars: &'static [u8], longest: &mut &'static str) {
        if chars.len() <= longest.len() {
            // cannot create a longer palindrome from remaining string
            return;
        }
        for i in 1..=chars.len() {
            if i > longest.len() && is_palindrome(&chars[0..i]) {
                *longest = str::from_utf8(&chars[0..i]).unwrap();
            }
        }
        longest_palindrome_helper(&chars[1..], longest);
    }

    let chars = s.leak().as_bytes();
    let longest = &mut str::from_utf8(&chars[0..1]).unwrap();
    longest_palindrome_helper(chars, longest);
    longest.to_owned()
}

// TODO: not done yet

// fn longest_palindrome(s: String) -> String {
//     let chars = s.into_bytes();

//     fn find_longest_palindrome(p: String, chars: &[u8], longest_palindrome: &mut &str) {
//         for i in 0..chars.len() {
//             if is_palindrome(p) && p.len() > longest_palindrome.len() {
//                 *longest_palindrome = p;
//             }

//             find_longest_palindrome(p + chars[0], &chars[1..], longest_palindrome);
//             find_longest_palindrome("", &chars[1..], longest_palindrome);
//         }
//     }

//     let mut res = String::from("");
//     find_longest_palindrome("".to_owned(), chars.as_slice(), &mut res);

//     todo!()
// }
