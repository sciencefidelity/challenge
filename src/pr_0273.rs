pub struct Solution;

// const ZERO: &str = "Zero";
// const DIGIT: &[&str] = &[
//     "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
// ];
// const TEEN: &[&str] = &[
//     "",
//     "Eleven",
//     "Twelve",
//     "Thirteen",
//     "Fourteen",
//     "Fifteen",
//     "Sixteen",
//     "Seventeen",
//     "Eighteen",
//     "Nineteen",
// ];
// const TEN: &[&str] = &[
//     "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
// ];
// const MULTIPLIER: &[&str] = &["Hundred", "Thousand", "Million", "Billion"];
//
// impl Solution {
//     pub fn number_to_words(num: i32) -> String {
//         let mut num = usize::try_from(num).unwrap();
//         if num == 0 {
//             return ZERO.to_owned();
//         }
//         let mut output = String::new();
//         let mut digits = Vec::new();
//         let mut multiplier = 0;
//         while num > 0 {
//             let rem = num % 10;
//             digits.push(rem);
//             num /= 10;
//         }
//         let mut i = 0;
//         let hundreds: &mut [usize; 3] = &mut [0; 3];
//         while i < digits.len() + digits.len() % 3 + 1 {
//             hundreds[i % 3] = *digits.get(i).unwrap_or(&0);
//             if i % 3 == 2 {
//                 let res = Self::handle_hundreds(hundreds);
//                 if output.is_empty() {
//                     if multiplier == 0 {
//                         output = res.join(" ");
//                     } else {
//                         output = res.join(" ") + " " + MULTIPLIER[multiplier];
//                     }
//                 } else {
//                     output = res.join(" ") + " " + MULTIPLIER[multiplier] + " " + &output;
//                 }
//                 multiplier += 1;
//             }
//             i += 1;
//         }
//
//         output
//     }
//
//     fn handle_hundreds(digits: &[usize]) -> Vec<&str> {
//         let mut res = Vec::new();
//         let mut i = 0;
//         while i < digits.len() {
//             let num = digits[i];
//             if i == 0 && num > 0 {
//                 if digits[i + 1] == 1 {
//                     res.push(TEEN[num]);
//                     i += 1;
//                 } else {
//                     res.push(DIGIT[num]);
//                 }
//             } else if i == 1 && num > 0 {
//                 res.push(TEN[num]);
//             }
//             if i == 2 && num > 0 {
//                 res.push(MULTIPLIER[0]);
//                 res.push(DIGIT[num]);
//             }
//             i += 1;
//         }
//         res.reverse();
//         res
//     }
// }

const NUMBER_TO_WORDS: &[(usize, &str); 31] = &[
    (1_000_000_000, "Billion"),
    (1_000_000, "Million"),
    (1000, "Thousand"),
    (100, "Hundred"),
    (90, "Ninety"),
    (80, "Eighty"),
    (70, "Seventy"),
    (60, "Sixty"),
    (50, "Fifty"),
    (40, "Forty"),
    (30, "Thirty"),
    (20, "Twenty"),
    (19, "Nineteen"),
    (18, "Eighteen"),
    (17, "Seventeen"),
    (16, "Sixteen"),
    (15, "Fifteen"),
    (14, "Fourteen"),
    (13, "Thirteen"),
    (12, "Twelve"),
    (11, "Eleven"),
    (10, "Ten"),
    (9, "Nine"),
    (8, "Eight"),
    (7, "Seven"),
    (6, "Six"),
    (5, "Five"),
    (4, "Four"),
    (3, "Three"),
    (2, "Two"),
    (1, "One"),
];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let num = usize::try_from(num).unwrap();
        if num == 0 {
            return "Zero".to_owned();
        }
        Self::helper(num)
    }

    fn helper(num: usize) -> String {
        for (val, word) in NUMBER_TO_WORDS {
            if num >= *val {
                let prefix = if num >= 100 {
                    &(Self::helper(num / *val) + " ")
                } else {
                    ""
                };
                let unit = *word;
                let suffix = if num % val == 0 {
                    ""
                } else {
                    &(" ".to_owned() + &Self::helper(num % val))
                };
                return [prefix, unit, suffix].concat();
            }
        }
        String::new()
    }
}

// const ZERO: &str = "Zero";
// const DIGITS: &[&str] = &[
//     "",
//     "One",
//     "Two",
//     "Three",
//     "Four",
//     "Five",
//     "Six",
//     "Seven",
//     "Eight",
//     "Nine",
//     "Ten",
//     "Eleven",
//     "Twelve",
//     "Thirteen",
//     "Fourteen",
//     "Fifteen",
//     "Sixteen",
//     "Seventeen",
//     "Eighteen",
//     "Nineteen",
// ];
// const TENS: &[&str] = &[
//     "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
// ];
// const MULTIPLIER: &[&str] = &["", "Thousand", "Million", "Billion"];
// const HUNDRED: &str = "Hundred";
//
// impl Solution {
//     pub fn number_to_words(num: i32) -> String {
//         let mut num = usize::try_from(num).unwrap();
//         if num == 0 {
//             return ZERO.to_owned();
//         }
//         let mut output = String::new();
//         for unit in MULTIPLIER {
//             if num % 1000 != 0 {
//                 output = format!("{}{} {}", Self::helper(num % 1000), unit, output);
//             }
//             num /= 1000;
//         }
//
//         output.trim_end().to_owned()
//     }
//
//     fn helper(n: usize) -> String {
//         if n == 0 {
//             String::new()
//         } else if n < 20 {
//             format!("{} ", DIGITS[n])
//         } else if n < 100 {
//             format!("{} {}", TENS[n / 10], Self::helper(n % 10))
//         } else {
//             format!("{} {} {}", DIGITS[n / 100], HUNDRED, Self::helper(n % 100))
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let expected = "Zero".to_owned();
        assert_eq!(expected, Solution::number_to_words(0));
    }

    #[test]
    fn case_1() {
        let expected = "One".to_owned();
        assert_eq!(expected, Solution::number_to_words(1));
    }

    #[test]
    fn case_2() {
        let expected = "Ten".to_owned();
        assert_eq!(expected, Solution::number_to_words(10));
    }

    #[test]
    fn case_3() {
        let expected = "Eleven".to_owned();
        assert_eq!(expected, Solution::number_to_words(11));
    }

    #[test]
    fn case_4() {
        let expected = "One Hundred".to_owned();
        assert_eq!(expected, Solution::number_to_words(100));
    }

    #[test]
    fn case_5() {
        let expected = "One Hundred One".to_owned();
        assert_eq!(expected, Solution::number_to_words(101));
    }

    #[test]
    fn case_6() {
        let expected = "One Hundred Eleven".to_owned();
        assert_eq!(expected, Solution::number_to_words(111));
    }

    #[test]
    fn case_7() {
        let expected = "One Hundred Twenty".to_owned();
        assert_eq!(expected, Solution::number_to_words(120));
    }

    #[test]
    fn case_8() {
        let expected = "One Hundred Twenty Three".to_owned();
        assert_eq!(expected, Solution::number_to_words(123));
    }

    #[test]
    fn case_9() {
        let expected = "One Thousand".to_owned();
        assert_eq!(expected, Solution::number_to_words(1000));
    }

    #[test]
    fn case_10() {
        let expected = "One Thousand One".to_owned();
        assert_eq!(expected, Solution::number_to_words(1001));
    }

    #[test]
    fn case_11() {
        let expected = "One Thousand Ten".to_owned();
        assert_eq!(expected, Solution::number_to_words(1010));
    }

    #[test]
    fn case_12() {
        let expected = "One Thousand Eleven".to_owned();
        assert_eq!(expected, Solution::number_to_words(1011));
    }

    #[test]
    fn case_13() {
        let expected = "One Thousand One Hundred".to_owned();
        assert_eq!(expected, Solution::number_to_words(1100));
    }

    #[test]
    fn case_14() {
        let expected = "One Thousand One Hundred Ten".to_owned();
        assert_eq!(expected, Solution::number_to_words(1110));
    }

    #[test]
    fn case_15() {
        let expected = "One Thousand One Hundred Eleven".to_owned();
        assert_eq!(expected, Solution::number_to_words(1111));
    }

    #[test]
    fn case_16() {
        let expected = "One Thousand One Hundred Twenty Three".to_owned();
        assert_eq!(expected, Solution::number_to_words(1123));
    }

    #[test]
    fn case_17() {
        let expected = "Twelve Thousand Three Hundred Forty Five".to_owned();
        assert_eq!(expected, Solution::number_to_words(12345));
    }

    #[test]
    fn case_18() {
        let expected =
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_owned();
        assert_eq!(expected, Solution::number_to_words(1234567));
    }

    #[test]
    fn case_19() {
        let expected = "Zero".to_owned();
        assert_eq!(expected, Solution::number_to_words(0));
    }

    #[test]
    fn case_20() {
        let expected = "One Hundred Twenty Three Million One Hundred Twenty Three Thousand One Hundred Twenty Three".to_owned();
        assert_eq!(expected, Solution::number_to_words(123123123));
    }

    #[test]
    fn case_21() {
        let expected = "One Billion One Hundred Twenty Three Million One Hundred Twenty Three Thousand One Hundred Twenty Three".to_owned();
        assert_eq!(expected, Solution::number_to_words(1123123123));
    }
}

// [3,2,1,3,2,1,3,2,1,1]
