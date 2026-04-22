/*Given two integers a and b, return the sum of the two integers without using the operators + and -.

Example 1:

Input: a = 1, b = 2
Output: 3
Example 2:

Input: a = 2, b = 3
Output: 5

*/

// res = res | (1 << i);
// res |= 1 << i;
//
// above are equivalent - << has a higher precedence than |

fn get_sum(mut a: i32, mut b: i32) -> i32 {
    let mut res = 0;
    let mut carry = false;
    for i in 0..32 {
        if a == 0 && b == 0 && !carry {
            break;
        }

        let a_ith = a & 0b1;
        let b_ith = b & 0b1;
        match (a_ith, b_ith) {
            (1, 1) => {
                if carry {
                    // already carrying 1 - flip bit
                    res |= 1 << i;
                }
                carry = true;
            }
            (1, 0) | (0, 1) => {
                if !carry {
                    res |= 1 << i;
                }
            }
            (0, 0) => {
                if carry {
                    res |= 1 << i;
                    carry = false;
                }
            }
            (_, _) => panic!("bit should be 0 or 1"),
        }

        a >>= 1;
        b >>= 1;

        dbg!(a, b, res, carry);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(get_sum(1, 2), 3);
        // assert_eq!(get_sum(1, 3), 4);
        // assert_eq!(get_sum(2, 3), 5);
        // assert_eq!(get_sum(20, 30), 50);
    }
}
