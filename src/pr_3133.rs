pub struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let (mut n, x) = (i64::from(n), i64::from(x));
        let mut result = x;
        while n > 1 {
            result = (result + 1) | x;
            n -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_end(3, 4), 6);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_end(2, 7), 15);
    }
}
