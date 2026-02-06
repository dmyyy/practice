// classic recursive fibonacci solution - chokes at higher levels of recursion
/* fn fib(mut n: u32) -> u32 {
    if n <= 2 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
} */

use std::collections::HashMap;

fn fib(mut n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut memo = HashMap::new();
    fib_helper(n, &mut memo)
}

fn fib_helper(mut n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    if n <= 2 {
        return 1;
    }
    let val = {
        let v1 = fib_helper(n - 1, memo);
        let v2 = fib_helper(n - 2, memo);
        v1 + v2
    };
    memo.insert(n, val);
    memo[&n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, fib(1));
        assert_eq!(1, fib(2));
        assert_eq!(2, fib(3));
        assert_eq!(3, fib(4));
        assert_eq!(5, fib(5));
    }
}
