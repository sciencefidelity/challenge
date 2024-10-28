use std::collections::HashSet;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let (mut longest_streak, mut unique_numbers) = (0, HashSet::with_capacity(nums.len()));
        for n in &nums {
            unique_numbers.insert(n);
        }
        for start_number in &nums {
            let (mut current, mut current_streak) = (*start_number, 0);
            while unique_numbers.contains(&current) {
                current_streak += 1;
                let Some(c) = current.checked_mul(current) else {
                    break;
                };
                current = c;
            }
            longest_streak = longest_streak.max(current_streak);
        }
        if longest_streak < 2 {
            -1
        } else {
            longest_streak
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
    }
}
