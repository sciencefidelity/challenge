pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut map = HashMap::new();
        for (idx, c) in s.chars().enumerate() {
            map.insert(c, idx as i32);
        }
        t.chars().enumerate().fold(0, |acc, (idx, c)| {
            if let Some(diff) = map.get(&c) {
                return acc + (diff - idx as i32).abs();
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_permutation_difference("abc".to_string(), "bac".to_string()),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()),
            12
        );
    }
}
