#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let (mut count, n, s) = (vec![0; 3], s.len() as i32, s.as_bytes());
        for b in s {
            count[usize::from(b - b'a')] += 1;
        }
        for b in &count {
            if *b < k {
                return -1;
            }
        }
        let (mut window, mut left, mut max_window) = (vec![0; 3], 0, 0);
        for right in 0..n {
            window[usize::from(s[right as usize] - b'a')] += 1;
            while left <= right && count[0] - window[0] < k
                || count[1] - window[1] < k
                || count[2] - window[2] < k
            {
                window[usize::from(s[left as usize] - b'a')] -= 1;
                left += 1;
            }
            max_window = max_window.max(right - left + 1);
        }
        n - max_window
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::take_characters("aabaaaacaabc".to_owned(), 2), 8);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::take_characters("a".to_owned(), 1), -1);
    }
}
