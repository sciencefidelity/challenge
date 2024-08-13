pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stringify(v: &[&str]) -> Vec<String> {
        v.into_iter().map(|&s| s.to_owned()).collect()
    }

    #[test]
    fn case_1() {
        assert_eq!(
            stringify(&["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]),
            Solution::letter_combinations("23".to_owned())
        );
    }
}
