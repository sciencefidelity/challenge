pub struct Solution;

impl Solution {
    pub const fn is_power_of_two(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(true, Solution::is_power_of_two(1));
    }

    #[test]
    fn case_2() {
        assert_eq!(true, Solution::is_power_of_two(16));
    }

    #[test]
    fn case_3() {
        assert_eq!(false, Solution::is_power_of_two(3));
    }
}
