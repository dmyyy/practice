
/*
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

 

Example 1:

Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.

Example 2:

Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.

Example 3:

Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.
*/

pub fn is_palindrome(s: String) -> bool {
    // remove non-alphanumeric (letters/numbers)
    let mut s: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    // convert to lower
    s = s.to_lowercase();

    if s.is_empty() {
        return true
    }
    let mut curr = 0;
    let mut end = s.len() - 1;    

    let s_bytes = s.as_bytes();

    while curr < end {
        if s_bytes[curr] != s_bytes[end] {
            return false
        }
        curr += 1;
        end -= 1;
    }
    true
}

mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(String::from("")), true);
        assert_eq!(is_palindrome(String::from(" ")), true);
        assert_eq!(is_palindrome(String::from("A man, a plan, a canal: Panama")), true);
        assert_eq!(is_palindrome(String::from("race a car")), false);
    }
}