use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_leaf: false,
        }
    }

    fn insert(&mut self, word: String) {
        word.chars()
            .fold(self, |node, c| node.children.entry(c).or_default())
            .is_leaf = true;
    }

    fn search(&self, word: String) -> bool {
        word.chars()
            .try_fold(self, |node, c| node.children.get(&c))
            .is_some_and(|n| n.is_leaf)
    }

    fn starts_with(&self, prefix: String) -> bool {
        prefix
            .chars()
            .try_fold(self, |node, c| node.children.get(&c))
            .is_some()
    }
}
