pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut decoded = String::new();
        let mut stack: Vec<char> = Vec::new();
        let mut iter = s.chars();
        let mut count: usize = 0;
        while let Some(c) = iter.next() {
            match c {
                c if c.is_numeric() => {
                    count = c.to_digit(10).unwrap().try_into().unwrap();
                }
                '[' => {
                    let mut tmp = String::new();
                    while let Some(c) = iter.next() {
                        if c == ']' {
                            break;
                        }
                        tmp.push(c);
                    }
                    let tmp = tmp.repeat(count);
                    decoded.push_str(&tmp);
                }
                c if c.is_alphabetic() => {
                    decoded.push(c);
                }
                _ => {}
            }
        }
        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            "aaabcbc".to_owned(),
            Solution::decode_string("3[a]2[bc]".to_owned())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            "accaccacc".to_owned(),
            Solution::decode_string("3[a2[c]]".to_owned())
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            "abcabccdcdcdef".to_owned(),
            Solution::decode_string("2[abc]3[cd]ef".to_owned())
        );
    }
}
