pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words
            .into_iter()
            .fold(0, |acc, x| acc + i32::from(x.starts_with(&pref)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::prefix_count(
                arr!["pay", "attention", "practice", "attend"],
                "at".to_owned()
            ),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::prefix_count(
                arr!["leetcode", "win", "loops", "success"],
                "code".to_owned()
            ),
            0
        );
    }
}
