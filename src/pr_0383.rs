pub struct Solution;

use std::collections::HashMap;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();
        for c in magazine.bytes() {
            *map.entry(c).or_insert(0) += 1;
        }
        for c in ransom_note.bytes() {
            match map.get_mut(&c) {
                Some(n) if *n > 0 => *n -= 1,
                _ => return false,
            };
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
            Solution::can_construct("a".to_owned(), "b".to_owned()),
            false
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "ab".to_owned()),
            false
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::can_construct("aa".to_owned(), "aab".to_owned()),
            true
        );
    }
}
