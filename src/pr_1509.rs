pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

// impl Solution {
//     pub fn min_difference(mut nums: Vec<i32>) -> i32 {
//         if nums.len() <= 4 {
//             return 0;
//         }
//         nums.sort_unstable();
//         (nums[nums.len() - 4] - nums[0])
//             .min(nums[nums.len() - 3] - nums[1])
//             .min(nums[nums.len() - 2] - nums[2])
//             .min(nums[nums.len() - 1] - nums[3])
//     }
// }

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let num_size = nums.len();
        if num_size <= 4 {
            return 0;
        }

        let mut max_heap = BinaryHeap::with_capacity(4);
        let mut min_heap = BinaryHeap::with_capacity(4);
        for num in nums {
            max_heap.push(Reverse(num));
            min_heap.push(num);
            if max_heap.len() > 4 {
                max_heap.pop();
                min_heap.pop();
            }
        }
        let largest_four = max_heap.into_sorted_vec();
        let smallest_four = min_heap.into_sorted_vec();

        let mut min_diff = i32::MAX;
        for i in 0..4 {
            min_diff = min_diff.min(largest_four[i].0 - smallest_four[3 - i]);
        }
        min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::min_difference(vec![3, 100, 20]), 0);
    }
}
