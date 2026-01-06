/*
Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

 Example 1:

 Input: digits = "23"
 Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 Example 2:

 Input: digits = "2"
 Output: ["a","b","c"]
 */

// accumulator pattern used again - cloning things around a lot
//

use std::collections::{HashMap, VecDeque};

pub fn letter_combinations(digits: String) -> Vec<String> {
    let nums_to_letters: HashMap<u32, Vec<char>> = HashMap::from([
        (2, vec!['a', 'b', 'c']),
        (3, vec!['d', 'e', 'f']),
        (4, vec!['g', 'h', 'i']),
        (5, vec!['j', 'k', 'l']),
        (6, vec!['m', 'n', 'o']),
        (7, vec!['p', 'q', 'r', 's']),
        (8, vec!['t', 'u', 'v']),
        (9, vec!['w', 'x', 'y', 'z']),
    ]);

    let chars = digits.chars();
    let mut combinations = VecDeque::<Vec<char>>::new();
    for char in chars {
        let num = char.to_digit(10).unwrap();
        combinations.push_back(nums_to_letters.get(&num).unwrap().clone());
    }
    let mut res = Vec::new();

    letter_combinations_helper(combinations, String::new(), &mut res);
    res
}

fn letter_combinations_helper(
    mut combinations: VecDeque<Vec<char>>,
    acc: String,
    res: &mut Vec<String>,
) {
    if combinations.is_empty() {
        res.push(acc);
    } else {
        for chars in combinations.pop_front() {
            for c in chars {
                let mut s = acc.clone();
                s.push(c);
                letter_combinations_helper(combinations.clone(), s, res);
            }
        }
    }
}
