pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn my_atoi(s: String) -> i32 {
        let mut output = String::new();
        for c in s.trim_start().chars() {
            match c {
                '-' if output.is_empty() => output.push('-'),
                '+' if output.is_empty() => output.push('+'),
                c if c.is_ascii_digit() => output.push(c),
                _ => break,
            }
        }
        match output.parse::<i32>() {
            Ok(val) => val,
            Err(message) => match message.kind() {
                std::num::IntErrorKind::PosOverflow => i32::MAX,
                std::num::IntErrorKind::NegOverflow => i32::MIN,
                _ => 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(42, Solution::my_atoi("42".to_owned()));
    }

    #[test]
    fn case_2() {
        assert_eq!(-42, Solution::my_atoi(" -042".to_owned()));
    }

    #[test]
    fn case_3() {
        assert_eq!(1337, Solution::my_atoi("1337c0d3".to_owned()));
    }

    #[test]
    fn case_4() {
        assert_eq!(0, Solution::my_atoi("0-1".to_owned()));
    }

    #[test]
    fn case_5() {
        assert_eq!(0, Solution::my_atoi("words and 987".to_owned()));
    }

    #[test]
    fn case_6() {
        assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_owned()));
    }

    #[test]
    fn case_7() {
        assert_eq!(0, Solution::my_atoi("+-12".to_owned()));
    }

    #[test]
    fn case_8() {
        assert_eq!(12, Solution::my_atoi("+12".to_owned()));
    }
}
