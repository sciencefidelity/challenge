pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let (str1, str2) = (str1.as_bytes(), str2.as_bytes());
        Self::iterate(str1, str2)
    }

    const fn iterate(str1: &[u8], str2: &[u8]) -> bool {
        let (len1, len2) = (str1.len(), str2.len());
        let (mut idx1, mut idx2) = (0, 0);
        while idx1 < len1 && idx2 < len2 {
            if str1[idx1] == str2[idx2] || Self::circular_inc(str1[idx1]) == str2[idx2] {
                idx2 += 1;
            }
            idx1 += 1;
        }
        idx2 == len2
    }

    const fn circular_inc(b: u8) -> u8 {
        if b == b'z' {
            return b'a';
        }
        b + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_make_subsequence(
            "abc".to_owned(),
            "ad".to_owned()
        ));
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_make_subsequence(
            "zc".to_owned(),
            "ad".to_owned()
        ));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::can_make_subsequence(
            "ab".to_owned(),
            "d".to_owned()
        ));
    }

    #[test]
    fn case_4() {
        assert!(Solution::can_make_subsequence(
            "dm".to_owned(),
            "e".to_owned()
        ));
    }
}
