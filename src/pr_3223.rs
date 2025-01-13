pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_length(s: String) -> i32 {
        let mut char_freq = vec![0; 26];
        for b in s.bytes() {
            char_freq[usize::from(b - b'a')] += 1;
        }
        let mut min_len = 0;
        for freq in char_freq {
            if freq == 0 {
                continue;
            }
            if freq % 2 == 0 {
                min_len += 2;
            } else {
                min_len += 1;
            }
        }
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimum_length("abaacbcbb".to_owned()), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimum_length("aa".to_owned()), 2);
    }
}
