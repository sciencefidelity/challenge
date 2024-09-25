#![allow(clippy::needless_pass_by_value)]
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    end_of_word: bool,
}

fn to_index(b: u8) -> usize {
    usize::from(b - b'a')
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for i in word.bytes().map(to_index) {
            node = node.children[i].get_or_insert(Box::new(Self::new()));
        }
        node.end_of_word = true;
    }

    fn node_for(&self, word: &str) -> Option<&Self> {
        let mut node = self;
        for i in word.bytes().map(to_index) {
            match node.children[i] {
                None => {
                    return None;
                }
                Some(ref child) => {
                    node = child;
                }
            }
        }
        Some(node)
    }

    fn search(&self, word: String) -> bool {
        self.node_for(&word).is_some_and(|node| node.end_of_word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.node_for(&prefix).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        assert!(trie.search("apple".to_owned()));
        assert!(!trie.search("app".to_owned()));
        assert!(trie.starts_with("app".to_owned()));
        trie.insert("app".to_owned());
        assert!(trie.search("app".to_owned()));
    }
}
