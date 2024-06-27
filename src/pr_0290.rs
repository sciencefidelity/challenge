pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.matches(' ').count() + 1 {
            return false;
        }
        let mut map = HashMap::new();
        let mut word_set = HashSet::new();
        for (word, c) in s.split_ascii_whitespace().zip(pattern.bytes()) {
            if let Some(w) = map.insert(c, word) {
                if w != word {
                    return false;
                }
            } else if !word_set.insert(word) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()),
            true
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned()),
            false
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned()),
            false
        );
    }
}
