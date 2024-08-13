pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn letter_combinations(_digits: String) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stringify(v: &[&str]) -> Vec<String> {
        v.iter().map(|&s| s.to_owned()).collect()
    }

    #[test]
    fn case_1() {
        assert_eq!(
            stringify(&["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]),
            Solution::letter_combinations("23".to_owned())
        );
    }
}
