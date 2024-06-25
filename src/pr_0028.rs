pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        for low in 0..=(haystack.len() - needle.len()) {
            if haystack[low..low + needle.len()] == needle {
                return low.try_into().unwrap();
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_owned(), "sad".to_owned()),
            0
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::str_str("leetcode".to_owned(), "leeto".to_owned()),
            -1
        );
    }
}
