pub struct Solution;

impl Solution {
    pub const fn reverse(mut x: i32) -> i32 {
        let mut rev: i32 = 0;
        while x != 0 {
            if let Some(num) = rev.checked_mul(10) {
                rev = num + x % 10;
            } else {
                return 0;
            }
            x /= 10;
        }
        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn case_2() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn case_3() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn case_4() {
        assert_eq!(0, Solution::reverse(i32::MAX));
    }

    #[test]
    fn case_5() {
        assert_eq!(0, Solution::reverse(i32::MIN));
    }

    #[test]
    fn case_6() {
        assert_eq!(1463847412, Solution::reverse(i32::MAX - 6));
    }

    #[test]
    fn case_7() {
        assert_eq!(-1463847412, Solution::reverse(i32::MIN + 7));
    }

    #[test]
    fn case_8() {
        assert_eq!(2147447412, Solution::reverse(2147447412));
    }

    #[test]
    fn case_9() {
        assert_eq!(-2147447412, Solution::reverse(-2147447412));
    }

    #[test]
    fn case_10() {
        assert_eq!(0, Solution::reverse(2147447422));
    }

    #[test]
    fn case_11() {
        assert_eq!(0, Solution::reverse(-2147447422));
    }

    #[test]
    fn case_12() {
        assert_eq!(963847412, Solution::reverse(214748369));
    }
}
