use std::{collections::VecDeque, iter};

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_strings(num1: String, num2: String) -> String {
        let (mut longer, mut shorter) = (&num1, &num2);
        if num2.len() > num1.len() {
            (longer, shorter) = (&num2, &num1);
        }
        let (mut buf, mut carry) = (VecDeque::with_capacity(longer.len() + 1), 0);
        for (a, b) in longer
            .bytes()
            .rev()
            .zip(shorter.bytes().rev().chain(iter::repeat(b'0')))
        {
            let sum = a - b'0' + b - b'0' + carry;
            buf.push_front(sum % 10 + b'0');
            carry = sum / 10;
        }
        String::from_utf8(buf.into()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::add_strings("11".to_owned(), "123".to_owned()),
            "134".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::add_strings("456".to_owned(), "77".to_owned()),
            "533".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::add_strings("0".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
    }
}
