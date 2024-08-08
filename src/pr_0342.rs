pub struct Solution;

impl Solution {
    pub const fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n & 0x5555_5555 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(true, Solution::is_power_of_four(16));
    }

    #[test]
    fn case_2() {
        assert_eq!(false, Solution::is_power_of_four(5));
    }

    #[test]
    fn case_3() {
        assert_eq!(true, Solution::is_power_of_four(1));
    }
}
