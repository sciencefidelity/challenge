pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.bytes().rfold(0, |acc, c| {
            acc + match c {
                b'I' if acc >= 5 => -1,
                b'I' => 1,
                b'V' => 5,
                b'X' if acc >= 50 => -10,
                b'X' => 10,
                b'L' => 50,
                b'C' if acc >= 500 => -100,
                b'C' => 100,
                b'D' => 500,
                _ => 1000,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_owned()), 58);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_owned()), 1994);
    }
}
