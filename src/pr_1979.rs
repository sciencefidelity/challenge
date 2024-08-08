pub struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        Self::gcd(
            nums.into_iter()
                .fold(None, |m: Option<(i32, i32)>, x| {
                    m.map_or(Some((x, x)), |(m1, m2)| Some((m1.min(x), m2.max(x))))
                })
                .unwrap(),
        )
    }

    const fn gcd((mut a, mut b): (i32, i32)) -> i32 {
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
        assert_eq!(2, Solution::find_gcd(vec![2, 5, 6, 9, 10]));
    }

    #[test]
    fn case_2() {
        assert_eq!(1, Solution::find_gcd(vec![7, 5, 6, 8, 3]));
    }

    #[test]
    fn case_3() {
        assert_eq!(3, Solution::find_gcd(vec![3, 3]));
    }
}
