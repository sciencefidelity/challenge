#![allow(clippy::float_cmp)]
pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let (mut result, mut base, mut exp) = (1.0, x, i64::from(n));
        if exp < 0 {
            base = 1.0 / base;
            exp = -exp;
        }
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            base *= base;
            exp /= 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }

    #[test]
    fn case_2() {
        assert_eq!((Solution::my_pow(2.1, 3) * 1000.0).trunc(), 9261.0);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::my_pow(100.0, 2), 10000.0);
    }
}
