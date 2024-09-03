pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_lucky(s: String, mut k: i32) -> i32 {
        let mut s = Self::string_to_number(&s.as_bytes());
        while k > 1 {
            s = Self::sum_digits(s);
            k -= 1;
        }
        s
    }

    fn string_to_number(s: &[u8]) -> i32 {
        let mut res = 0;
        for n in s.iter().map(|b| i32::from(*b) - 96).rev() {
            res += Self::sum_digits(n);
        }
        res
    }

    const fn sum_digits(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n % 10;
            n /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::get_lucky("iiii".to_owned(), 1), 36);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::get_lucky("leetcode".to_owned(), 2), 6);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::get_lucky("zbax".to_owned(), 2), 8);
    }
}
