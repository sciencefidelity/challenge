pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();
        let mut merged = String::new();
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(char1), Some(char2)) => {
                    merged.push(char1);
                    merged.push(char2);
                }
                (Some(char1), None) => merged.push(char1),
                (None, Some(char2)) => merged.push(char2),
                _ => return merged,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
