/*

Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

Example 1:

Input: x = 123
Output: 321
Example 2:

Input: x = -123
Output: -321
Example 3:

Input: x = 120
Output: 21
*/

// wrapping - wraps around instead of overfliow
// saturating - clamp to min/max
// checked - returns option on overflow
// overflowing - returns a result if overflow occurs
// % - gives you remainder and keeps sign
// rem_euclid if you want to keep the sign

fn reverse(mut x: i32) -> i32 {
    let mut res: i32 = 0;
    while x != 0 {
        let digit = x % 10;
        match res.checked_mul(10).and_then(|r| r.checked_add(digit)) {
            Some(v) => res = v,
            None => return 0,
        }
        x /= 10;
    }
    res
}
