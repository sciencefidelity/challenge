pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn fraction_addition(expression: String) -> String {
        let (mut numer, mut denom) = (0, 1);
        let mut iter = expression.chars().peekable();
        let mut c = iter.next().unwrap();
        while iter.peek().is_some() {
            let (mut curr_numer, mut curr_denom, mut is_negative) = (0, 0, false);
            if c == '-' || c == '+' {
                if c == '-' {
                    is_negative = true;
                }
                c = iter.next().unwrap();
            }
            while c.is_ascii_digit() {
                let val = c.to_digit(10).unwrap();
                curr_numer = curr_numer * 10 + i32::try_from(val).unwrap();
                c = iter.next().unwrap();
            }
            if is_negative {
                curr_numer *= -1;
            }
            c = iter.next().unwrap();
            while c.is_ascii_digit() {
                let val = c.to_digit(10).unwrap();
                curr_denom = curr_denom * 10 + i32::try_from(val).unwrap();
                if iter.peek().is_some() {
                    c = iter.next().unwrap();
                } else {
                    break;
                }
            }
            numer = numer * curr_denom + curr_numer * denom;
            denom *= curr_denom;
        }
        let gcd = Self::gcd(numer, denom).abs();
        numer /= gcd;
        denom /= gcd;
        format!("{numer}/{denom}")
    }

    const fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2".to_owned()),
            "0/1".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2+1/3".to_owned()),
            "1/3".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::fraction_addition("1/3-1/2".to_owned()),
            "-1/6".to_owned()
        );
    }
}
