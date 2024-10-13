use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pq = BinaryHeap::with_capacity(nums.len());
        let (mut max_val, mut range_start, mut range_end) = (i32::MIN, 0, i32::MAX);
        for (i, num) in nums.iter().enumerate() {
            pq.push(Reverse((num[0], (i, 0))));
            max_val = max_val.max(num[0]);
        }
        while pq.len() == nums.len() {
            if let Some(Reverse((min_val, indicies))) = pq.pop() {
                let (row, col) = (indicies.0, indicies.1);
                if max_val - min_val < range_end - range_start {
                    (range_start, range_end) = (min_val, max_val);
                }
                if col + 1 < nums[row].len() {
                    let next_val = nums[row][col + 1];
                    pq.push(Reverse((next_val, (row, col + 1))));
                    max_val = max_val.max(next_val);
                }
            }
        }
        vec![range_start, range_end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::smallest_range(arr![[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]]),
            vec![20, 24]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::smallest_range(arr![[1, 2, 3], [1, 2, 3], [1, 2, 3]]),
            vec![1, 1]
        );
    }
}
