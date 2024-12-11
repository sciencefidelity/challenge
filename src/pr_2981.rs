#![allow(unused_assignments)]
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn maximum_length(s: String) -> i32 {
        let mut count = HashMap::new();
        let mut substring_length = 0;
        let s = s.as_bytes();
        for start in 0..s.len() {
            let byte = s[start];
            substring_length = 0;
            for end in s.iter().skip(start) {
                if byte == *end {
                    substring_length += 1;
                    count
                        .entry((byte, substring_length))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                } else {
                    break;
                }
            }
        }
        let mut ans = 0;
        for (k, v) in count {
            if v >= 3 && k.1 > ans {
                ans = k.1;
            }
        }
        if ans == 0 {
            -1
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::maximum_length("aaaa".to_owned()), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_length("abcdef".to_owned()), -1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::maximum_length("abcaba".to_owned()), 1);
    }
}
