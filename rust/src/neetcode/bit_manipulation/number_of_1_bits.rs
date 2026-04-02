/*
You are given an unsigned integer n. Return the number of 1 bits in its binary representation.

Example 1:

Input: n = 00000000000000000000000000010111

Output: 4
Example 2:

Input: n = 01111111111111111111111111111101

Output: 30
*/

fn hamming_weight(mut n: u32) -> u32 {
    let mut count = 0;

    // normal solution
    /*
    for _ in 0..32 {
        if n % 2 == 1 {
            count += 1;
        }
        n = n >> 1;
    }
    */

    // clever solution
    while n != 0 {
        n = n & (n - 1);
        count += 1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(hamming_weight(0b00000000000000000000000000010111), 4);
        assert_eq!(hamming_weight(0b01111111111111111111111111111101), 30);
    }
}
