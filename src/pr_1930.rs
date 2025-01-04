#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut first: Vec<i32> = vec![-1; 26];
        let mut last: Vec<i32> = vec![-1; 26];
        for (i, b) in s.iter().enumerate() {
            let curr = b - b'a';
            if first[usize::from(curr)] == -1 {
                first[usize::from(curr)] = i as i32;
            }
            last[usize::from(curr)] = i as i32;
        }
        let mut ans: i32 = 0;
        for i in 0..26 {
            if first[i] == -1 {
                continue;
            }
            let mut between = HashSet::new();
            for j in (first[i] + 1)..last[i] {
                between.insert(s[j as usize]);
            }
            ans += between.len() as i32;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::count_palindromic_subsequence("aabca".to_owned()),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::count_palindromic_subsequence("adc".to_owned()), 0);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::count_palindromic_subsequence("bbcbaba".to_owned()),
            4
        );
    }
}
