use std::ops::Mul;

pub struct Solution;

struct Integer(String);

impl Mul for Integer {
    type Output = String;

    fn mul(self, rhs: Self) -> Self::Output {
        let (len1, len2) = (self.0.len(), rhs.0.len());
        let digits = {
            let mut digits = vec![0; len1 + len2];
            for (idx1, b1) in self.0.bytes().rev().enumerate() {
                for (idx2, b2) in rhs.0.bytes().rev().enumerate() {
                    let tmp = (b1 - b'0') * (b2 - b'0') + digits[idx1 + idx2];
                    digits[idx1 + idx2] = tmp % 10;
                    digits[idx1 + idx2 + 1] += tmp / 10;
                }
            }
            while digits.len() > 1 && digits.last() == Some(&0) {
                digits.pop();
            }
            digits
        };
        digits.into_iter().rev().map(|d| d.to_string()).collect()
    }
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        Integer(num1) * Integer(num2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::multiply("2".to_owned(), "3".to_owned()),
            "6".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::multiply("123".to_owned(), "456".to_owned()),
            "56088".to_owned()
        );
    }
}
