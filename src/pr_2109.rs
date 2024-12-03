#![allow(clippy::needless_pass_by_value, clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = String::with_capacity(s.len() + spaces.len());
        let mut last = 0;
        for space in spaces {
            result.push_str(&s[last as usize..space as usize]);
            result.push(' ');
            last = space;
        }
        result.push_str(&s[last as usize..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_owned(), vec![8, 13, 15]),
            "Leetcode Helps Me Learn".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::add_spaces("icodeinpython".to_owned(), vec![1, 5, 7, 9]),
            "i code in py thon".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::add_spaces("spacing".to_owned(), vec![0, 1, 2, 3, 4, 5, 6]),
            " s p a c i n g".to_owned()
        );
    }
}
