pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman = String::new();
        let map = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        for (symbol, value) in map {
            while num >= value {
                roman.push_str(symbol);
                num -= value;
            }
        }
        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX".to_owned());
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_owned());
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_owned());
    }
}
