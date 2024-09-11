pub struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_bit_flips(3, 4), 3);
    }
}
