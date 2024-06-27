pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();
        for (a, b) in s.bytes().zip(t.bytes()) {
            match (&s_map.get(&a), &t_map.get(&b)) {
                (None, None) => {
                    s_map.insert(a, b);
                    t_map.insert(b, a);
                }
                (Some(_), None) | (None, Some(_)) => return false,
                (Some(c), Some(d)) if &a != *d || &b != *c => return false,
                _ => continue,
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
            Solution::is_isomorphic("egg".to_owned(), "add".to_owned()),
            true
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()),
            false
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::is_isomorphic("paper".to_owned(), "title".to_owned()),
            true
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::is_isomorphic("badc".to_owned(), "baba".to_owned()),
            false
        );
    }
}
