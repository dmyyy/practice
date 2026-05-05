pub fn my_atoi(s: String) -> i32 {
    // trim leading whitespace
    let s = s.trim_start();

    if s.is_empty() {
        return 0;
    }

    let mut idx = 0;
    let chars = s.as_bytes();
    let sign = match chars[0] {
        b'-' => {
            idx += 1;
            -1
        }
        b'+' => {
            idx += 1;
            1
        }
        _ => 1,
    };

    // skip leading zeros
    while idx < chars.len() && chars[idx] == b'0' {
        idx += 1;
    }

    let mut res: i32 = 0;
    while idx < chars.len() && chars[idx].is_ascii_digit() {
        res = res.saturating_mul(10);
        res = res.saturating_add(sign * (chars[idx] - b'0') as i32);
        idx += 1;
    }
    return res;
}
