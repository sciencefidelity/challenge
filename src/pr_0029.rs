pub struct Solution;

impl Solution {
    pub const fn divide(dividend: i32, divisor: i32) -> i32 {
        dividend.saturating_div(divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::divide(-2_147_483_648, -1), 2_147_483_647);
    }
}
