pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_deque = VecDeque::new();
        let mut min_deque = VecDeque::new();
        let mut left = 0;
        let mut max_length = 0;
        for right in 0..nums.len() {
            while !max_deque.is_empty() && *max_deque.back().unwrap() < nums[right] {
                max_deque.pop_back();
            }
            max_deque.push_back(nums[right]);

            while !min_deque.is_empty() && *min_deque.back().unwrap() > nums[right] {
                min_deque.pop_back();
            }
            min_deque.push_back(nums[right]);

            while max_deque.front().unwrap() - min_deque.front().unwrap() > limit {
                if *max_deque.front().unwrap() == nums[left] {
                    max_deque.pop_front();
                }
                if *min_deque.front().unwrap() == nums[left] {
                    min_deque.pop_front();
                }
                left += 1;
            }
            max_length = max_length.max(right - left + 1);
        }
        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_subarray(vec![8, 2, 4, 7], 4), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0),
            3
        );
    }
}
