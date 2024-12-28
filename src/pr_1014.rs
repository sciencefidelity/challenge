#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let (mut max_left_score, mut max_score) = (vec![0; values.len()], 0);
        max_left_score[0] = values[0];
        for (i, v) in values.into_iter().enumerate().skip(1) {
            let current_right_score = v - i as i32;
            max_score = max_score.max(max_left_score[i - 1] + current_right_score);
            let current_left_score = v + i as i32;
            max_left_score[i] = current_left_score.max(max_left_score[i - 1]);
        }
        max_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]),
            11
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
