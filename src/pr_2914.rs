pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_changes(s: String) -> i32 {
        let s = s.as_bytes();
        let mut current_byte = s[0];
        let (mut consecutive_count, mut min_changes_required) = (0, 0);
        for b in s {
            if *b == current_byte {
                consecutive_count += 1;
                continue;
            }
            if consecutive_count % 2 == 0 {
                consecutive_count = 1;
            } else {
                consecutive_count = 0;
                min_changes_required += 1;
            }
            current_byte = *b;
        }
        min_changes_required
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_changes("1001".to_owned()), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_changes("10".to_owned()), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_changes("0000".to_owned()), 0);
    }
}
