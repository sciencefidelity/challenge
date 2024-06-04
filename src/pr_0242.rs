pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = HashMap::<char, u32>::new();
        for c in s.chars() {
            match map.get(&c) {
                Some(i) => map.insert(c, i + 1),
                None => map.insert(c, 1),
            };
        }
        for c in t.chars() {
            match map.get(&c) {
                Some(i) => {
                    if *i > 1 {
                        map.insert(c, i - 1);
                    } else {
                        map.remove(&c);
                    }
                }
                None => return false,
            }
        }
        map.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
            true
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::is_anagram("rat".to_owned(), "car".to_owned()),
            false
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::is_anagram("ab".to_owned(), "a".to_owned()), false);
    }
}
