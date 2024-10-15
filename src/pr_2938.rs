pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_steps(s: String) -> i64 {
        let (mut result, mut zeros) = (0, 0);
        for b in s.bytes().rev() {
            match b {
                b'0' => zeros += 1,
                _ => result += zeros,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimum_steps("101".to_owned()), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimum_steps("100".to_owned()), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::minimum_steps("0111".to_owned()), 0);
    }
}
