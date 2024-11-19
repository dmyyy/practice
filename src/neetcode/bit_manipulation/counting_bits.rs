/*
Given an integer n, count the number of 1's in the binary representation of every number in the range [0, n].

Return an array output where output[i] is the number of 1's in the binary representation of i.

Example 1:

Input: n = 4

Output: [0,1,1,2,1]
Explanation:
0 --> 0
1 --> 1
2 --> 10
3 --> 11
4 --> 100
*/

// brute force
/*
fn count_bits(n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(n as usize);
    for i in 0..=n {
        let mut num = i;
        let mut count = 0;
        for _ in 0..32 {
            if num % 2 == 1 {
                count += 1;
            }
            num = num >> 1;
        }
        res.push(count);
    }
    res
}
*/

// memoized solution
fn count_bits(n: u32) -> Vec<u32> {
    let mut res = Vec::with_capacity(n as usize);
    res.push(0);

    // msb we've reached so far
    let mut msb = 1;
    for i in 1..=n {
        if i == 2 * msb {
            msb = i;
        }

        res.push(1 + res[(i - msb) as usize]);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_bits(4), vec![0, 1, 1, 2, 1]);
    }
}
