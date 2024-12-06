use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, mut max_sum: i32) -> i32 {
        let banned: HashSet<i32> = HashSet::from_iter(banned);
        let mut count = 0;
        for num in 1..=n {
            if banned.contains(&num) {
                continue;
            }
            max_sum -= num;
            if max_sum < 0 {
                return count;
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_count(vec![11], 7, 50), 7);
    }
}
