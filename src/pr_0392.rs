pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s == t {
            return true;
        }
        let mut iter = s.chars().peekable();
        for c1 in t.chars() {
            if let Some(c2) = iter.peek() {
                if &c1 == c2 {
                    iter.next().unwrap();
                }
            }
            if iter.peek().is_none() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::is_subsequence(
            "abc".to_owned(),
            "ahbgdc".to_owned()
        ),);
    }

    #[test]
    fn case_2() {
        assert!(!Solution::is_subsequence(
            "axc".to_owned(),
            "ahbgdc".to_owned()
        ),);
    }

    #[test]
    fn case_3() {
        assert!(Solution::is_subsequence(String::new(), String::new()));
    }
}
