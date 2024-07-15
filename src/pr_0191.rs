pub struct Solution;

impl Solution {
    pub const fn hamming_weight(mut n: i32) -> i32 {
        let mut ones = 0;
        while n != 0 {
            n &= n - 1;
            ones += 1;
        }
        ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::hamming_weight(11), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::hamming_weight(128), 1);
    }
}
