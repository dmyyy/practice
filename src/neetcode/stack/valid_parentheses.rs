/*
You are given a string s consisting of the following characters: '(', ')', '{', '}', '[' and ']'.

The input string s is valid if and only if:

Every open bracket is closed by the same type of close bracket.
Open brackets are closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.
Return true if s is a valid string, and false otherwise.

Example 1:

Input: s = "[]"

Output: true
Example 2:

Input: s = "([{}])"

Output: true
Example 3:

Input: s = "[(])"

Output: false
Explanation: The brackets are not closed in the correct order.
*/

fn is_valid(s: String) -> bool {
    let mut stack = Vec::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if Some(c) != stack.pop() {
                    return false;
                }
            }
            _ => panic!("unexpected character"),
        };
    }
    // if stack isn't empty - we have an unclosed parentheses
    stack.is_empty()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert!(is_valid("".to_string()));
    }

    #[test]
    fn test_single_pairs_valid() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("[]".to_string()));
        assert!(is_valid("{}".to_string()));
    }

    #[test]
    fn test_multiple_pairs_valid() {
        assert!(is_valid("()[]{}".to_string()));
        assert!(is_valid("([]){}".to_string()));
    }

    #[test]
    fn test_nested_valid() {
        assert!(is_valid("([{}])".to_string()));
        assert!(is_valid("{[()()]}".to_string()));
    }

    #[test]
    fn test_invalid_mismatch() {
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("[(])".to_string()));
        assert!(!is_valid("([)]".to_string()));
    }

    #[test]
    fn test_extra_closing_or_unmatched_closing() {
        assert!(!is_valid("([{}])]".to_string()));
        assert!(!is_valid(")(" .to_string()));
        assert!(!is_valid("]".to_string()));
    }

    #[test]
    fn test_only_opening_brackets() {
        assert!(!is_valid("[".to_string()));
        assert!(!is_valid("(((".to_string()));
    }

    #[test]
    fn test_long_balanced_string() {
        let mut s = String::new();
        for _ in 0..1000 {
            s.push('[');
        }
        for _ in 0..1000 {
            s.push(']');
        }
        assert!(is_valid(s));
    }

    #[test]
    #[should_panic(expected = "unexpected character")]
    fn test_unexpected_character_panics() {
        is_valid("a".to_string());
    }
}
