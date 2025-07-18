/*
Design an algorithm to encode a list of strings to a single string. The encoded string is then decoded back to the original list of strings.

Please implement encode and decode

Example 1:

Input: ["neet","code","love","you"]

Output:["neet","code","love","you"]
Example 2:

Input: ["we","say",":","yes"]

Output: ["we","say",":","yes"]
*/

// naive solution - use ascii char as delimiter
// doesn't work - strs is utf-8 so its hard to find a delimiter that works here

// fn encode(strs: &[&str]) -> String {
//     strs.join("|")
// }

// fn decode(s: &str) -> Vec<&str> {
//     s.split('|').collect()
// }

// encode using the length of each string

const DELIMITER: char = '#';

fn encode(strs: &[&str]) -> String {
    let mut res = String::new();

    if strs.is_empty() {
        return res;
    }

    for &s in strs {
        res.push_str(&s.len().to_string());
        res.push(DELIMITER);
        res.push_str(s);
    }
    res
}

fn decode(mut s: String) -> Vec<String> {
    let mut res = Vec::new();
    while let Some(pos) = s.find(DELIMITER) {
        let len: usize = s[0..pos].parse().expect("invalid length");

        // push token
        res.push(s[pos + 1..pos + 1 + len].to_string());

        // shrink string
        s = s.split_off(pos + 1 + len);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec!["neet", "code", "loves", "you"];

        let encoded_str = encode(&input);
        // println!("{}", encoded_str);

        let decoded_str = decode(encoded_str);
        // println!("{:?}", decoded_str);

        assert_eq!(input, decoded_str);
    }
}
