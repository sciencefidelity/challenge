pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut l, mut r) = (1, *piles.iter().max().unwrap());
        while l < r {
            let k = (l + r) / 2;
            if Self::can_finish(k, h, &piles) {
                r = k;
            } else {
                l = k + 1;
            }
        }
        l
    }

    fn can_finish(k: i32, mut h: i32, piles: &[i32]) -> bool {
        for pile in piles {
            h -= (pile + k - 1) / k;
            if h < 0 {
                break;
            }
        }
        h >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
