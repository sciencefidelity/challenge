pub struct Solution;

impl Solution {
    pub const fn is_palindrome(x: i32) -> bool {
        if x == 0 {
            return true;
        }
        if x < 0 || x % 10 == 0 {
            return false;
        }
        let mut num = x;
        let mut rev = 0;
        while num > 0 {
            rev = rev * 10 + num % 10;
            num /= 10;
        }
        rev == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
