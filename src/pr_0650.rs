pub struct Solution;

impl Solution {
    pub const fn min_steps(mut n: i32) -> i32 {
        let (mut ans, mut d) = (0, 2);
        while n > 1 {
            while n % d == 0 {
                ans += d;
                n /= d;
            }
            d += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(3, Solution::min_steps(3));
    }

    #[test]
    fn case_2() {
        assert_eq!(0, Solution::min_steps(1));
    }

    #[test]
    fn case_3() {
        assert_eq!(7, Solution::min_steps(7));
    }

    #[test]
    fn case_4() {
        assert_eq!(7, Solution::min_steps(10));
    }

    #[test]
    fn case_5() {
        assert_eq!(9, Solution::min_steps(24));
    }

    #[test]
    fn case_6() {
        assert_eq!(21, Solution::min_steps(1000));
    }
}
