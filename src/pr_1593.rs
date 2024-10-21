use std::collections::HashSet;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_unique_split(s: String) -> i32 {
        let mut seen = HashSet::new();
        Self::backtrack(s.as_bytes(), 0, &mut seen)
    }

    fn backtrack(s: &[u8], start: usize, seen: &mut HashSet<Vec<u8>>) -> i32 {
        if start == s.len() {
            return 0;
        }
        let mut max_count = 0;
        for end in start + 1..=s.len() {
            let substring = &s[start..end];
            if !seen.contains(substring) {
                seen.insert(substring.to_vec());
                max_count = max_count.max(1 + Self::backtrack(s, end, seen));
                seen.remove(substring);
            }
        }
        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_unique_split("ababccc".to_owned()), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_unique_split("aba".to_owned()), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_unique_split("aa".to_owned()), 1);
    }
}
