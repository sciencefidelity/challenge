pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() == goal.len() {
            (s.clone() + &s).contains(&goal)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::rotate_string(
            "abcde".to_owned(),
            "cdeab".to_owned()
        ));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::rotate_string(
            "abcde".to_owned(),
            "abced".to_owned()
        ));
    }
}
