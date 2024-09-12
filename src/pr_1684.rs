pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mask = allowed.bytes().fold(0, |acc, b| acc + (1 << (b - b'a')));
        words.into_iter().fold(0, |acc, w| {
            acc + i32::from(w.bytes().all(|b| (mask >> (b - b'a')) & 1 != 0))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::count_consistent_strings(
                "ab".to_owned(),
                arr!["ad", "bd", "aaab", "baa", "badab"]
            ),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::count_consistent_strings(
                "abc".to_owned(),
                arr!["a", "b", "c", "ab", "ac", "bc", "abc"]
            ),
            7
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::count_consistent_strings(
                "cad".to_owned(),
                arr!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]
            ),
            4
        );
    }
}
