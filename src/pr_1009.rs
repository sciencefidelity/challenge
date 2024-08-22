pub struct Solution;

impl Solution {
    pub const fn bitwise_complement(n: i32) -> i32 {
        let mut i = 1;
        while i < n {
            i = (i << 1) + 1;
        }
        i - n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::bitwise_complement(5), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::bitwise_complement(7), 0);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::bitwise_complement(10), 5);
    }
}
