use crate::neetcode::trees::test_helpers;

fn fib(n: usize) -> u32 {
    let mut nums = vec![0; n + 1];
    nums[1] = 1;
    for i in 0..n {
        if i + 1 < nums.len() {
            nums[i + 1] += nums[i];
        }
        if i + 2 < nums.len() {
            nums[i + 2] += nums[i];
        }
    }
    nums[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fib(6), 8);
    }
}
