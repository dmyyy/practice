/*

You are given an array of strings tokens that represents a valid arithmetic expression in Reverse Polish Notation.

Return the integer that represents the evaluation of the expression.

The operands may be integers or the results of other operations.
The operators include '+', '-', '*', and '/'.
Assume that division between integers always truncates toward zero.
Example 1:

Input: tokens = ["1","2","+","3","*","4","-"]

Output: 5

Explanation: ((1 + 2) * 3) - 4 = 5

*/

use std::str::FromStr;

fn eval_rpn(tokens: Vec<String>) -> i32 {
    // can solve this using a stack..

    let mut s = Vec::new();
    for t in tokens {
        if let Ok(n) = i32::from_str(t.as_str()) {
            // number - push on stack
            s.push(n);
        } else {
            // operator - pop last two numbers from the stack and apply operator to numbers
            let n1 = s.pop().unwrap();
            let n2 = s.pop().unwrap();
            s.push(match t.as_str() {
                "+" => n2 + n1,
                "-" => n2 - n1,
                "*" => n2 * n1,
                "/" => n2 / n1,
                _ => panic!("unexpected operator"),
            });
        }

        dbg!(t, &s);
    }

    // stack should coalesce down to one value
    assert!(s.len() == 1);

    return s.pop().unwrap();
}
