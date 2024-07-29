pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let n = i32::try_from(rating.len()).unwrap();
        let mut teams = 0;
        for mid in 0..n {
            let (mut left_smaller, mut right_larger) = (0, 0);
            let mut left = mid - 1;
            while left >= 0 {
                if rating[usize::try_from(left).unwrap()] < rating[usize::try_from(mid).unwrap()] {
                    left_smaller += 1;
                }
                left -= 1;
            }
            for right in (mid + 1)..n {
                if rating[usize::try_from(right).unwrap()] > rating[usize::try_from(mid).unwrap()] {
                    right_larger += 1;
                }
            }
            teams += left_smaller * right_larger;
            let left_larger = mid - left_smaller;
            let right_smaller = n - mid - 1 - right_larger;
            teams += left_larger * right_smaller;
        }
        teams
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(3, Solution::num_teams(vec![2, 5, 3, 4, 1]));
    }

    #[test]
    fn case_2() {
        assert_eq!(0, Solution::num_teams(vec![2, 1, 3]));
    }

    #[test]
    fn case_3() {
        assert_eq!(4, Solution::num_teams(vec![1, 2, 3, 4]));
    }
}
