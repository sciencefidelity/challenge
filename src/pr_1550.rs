pub struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut consecutive_odds = 0;
        for item in arr {
            if Self::is_odd(item) {
                consecutive_odds += 1;
            } else {
                consecutive_odds = 0;
            }
            if consecutive_odds >= 3 {
                return true;
            }
        }
        false
    }

    const fn is_odd(num: i32) -> bool {
        num % 2 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 8, 23, 12]),
            false
        );
    }
}
