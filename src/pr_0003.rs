use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let (mut left, mut right, mut answer) = (-1, 0, 0);
        s.bytes().for_each(|b| {
            if let Some(last) = map.insert(b, right) {
                left = left.max(last);
            }
            answer = answer.max(right - left);
            right += 1;
        });
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
    }
}
