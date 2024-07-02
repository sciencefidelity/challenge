pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut longest_palindrome_length = 0;
        let mut set = HashSet::new();
        for b in s.bytes() {
            if set.contains(&b) {
                longest_palindrome_length += 2;
                set.remove(&b);
            } else {
                set.insert(b);
            }
        }
        if !set.is_empty() {
            longest_palindrome_length += 1;
        }
        longest_palindrome_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_owned()), 7);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::longest_palindrome("a".to_owned()), 1);
    }
}
