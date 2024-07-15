pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .bytes()
            .rev()
            .take_while(|&b| b != b' ')
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_owned()),
            4
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_owned()),
            6
        );
    }
}
