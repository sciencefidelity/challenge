pub struct Solution;

impl Solution {
    pub const fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(false, Solution::can_win_nim(4));
    }

    #[test]
    fn case_2() {
        assert_eq!(true, Solution::can_win_nim(1));
    }

    #[test]
    fn case_3() {
        assert_eq!(true, Solution::can_win_nim(2));
    }
}
