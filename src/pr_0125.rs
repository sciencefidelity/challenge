pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if !s.as_bytes()[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            if !s.as_bytes()[right].is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }
            if s.as_bytes().to_ascii_lowercase()[left] != s.as_bytes().to_ascii_lowercase()[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_4() {
        assert_eq!(Solution::is_palindrome("      ".to_string()), true);
    }
    #[test]
    fn case_5() {
        assert_eq!(Solution::is_palindrome("abba".to_string()), true);
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}