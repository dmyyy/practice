use std::collections::HashMap;

// other rust solutions do Opt<Box<Node>> - rust makes this stuff kinda hard gonna try doing it again in golang

#[derive(Default)]
struct WordDictionary {
    children: HashMap<u8, WordDictionary>,
    is_leaf: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn add_word(&mut self, word: String) {
        if word.is_empty() {
            return;
        }

        let mut chars = word.as_bytes();
        let mut dict = self.children.entry(chars[0]).or_default();
        chars = &chars[1..];
        while !chars.is_empty() {
            dict = dict.children.entry(chars[0]).or_default();
            chars = &chars[1..];
        }
        dict.is_leaf = true;
    }

    fn search(&self, word: String) -> bool {
        false
    }

    // attempted to handle this iteratively but it was kind of impossible - need to look for a clean recursive solution...
    /*
    fn search(&self, word: String) -> bool {
        let mut res = false;
        fn search_helper(mut dict: &WordDictionary, mut chars: &[u8]) -> bool {
            while !chars.is_empty() {
                if chars[0] == b'.' {
                    for (_, dict) in dict.children.iter() {
                        return search_helper(dict, &chars[1..]);
                    }
                } else if let Some(next) = dict.children.get(&chars[0]) {
                    dict = next;
                    chars = &chars[1..];
                } else {
                    return false;
                };
            }
            return dict.is_leaf;
        }

        let mut chars = word.as_bytes();
        if chars[0] == b'.' {
            for (_, dict) in self.children.iter() {
                res |= search_helper(dict, &chars[1..]);
            }
        } else if let Some(mut dict) = self.children.get(&chars[0]) {
            chars = &chars[1..];
            while !chars.is_empty() {
                if chars[0] == b'.' {
                    for (_, dict) in dict.children.iter() {
                        res |= search_helper(dict, &chars[1..]);
                    }
                } else if let Some(next) = self.children.get(&chars[0]) {
                    dict = next;
                    chars = &chars[1..];
                } else {
                    return false;
                };
            }
            res |= dict.is_leaf;
        } else {
            return false;
        }
        return res;
    }
    */
}
