pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let reversed_string: Vec<u8> = s.bytes().rev().collect();
        let len = s.len();
        for i in 0..len {
            if *s[..len - i].as_bytes() == reversed_string[i..] {
                return String::from_utf8(reversed_string[..i].to_vec()).unwrap() + &s;
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_owned()),
            "aaacecaaa".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_owned()),
            "dcbabcd".to_owned()
        );
    }
}
