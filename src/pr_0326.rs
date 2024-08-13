pub struct Solution;

impl Solution {
    pub const fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1_162_261_467 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::is_power_of_three(27));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::is_power_of_three(0));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_power_of_three(-1));
    }
}
