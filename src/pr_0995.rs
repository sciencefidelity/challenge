pub struct Solution;

// impl Solution {
//     pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
//         let k = k as usize;
//         let mut flipped = vec![false; nums.len()];
//         let mut valid_flipps_from_past_window = 0;
//         let mut flip_count = 0;
//
//         for i in 0..nums.len() {
//             if i >= k {
//                 if flipped[i - k] {
//                     valid_flipps_from_past_window -= 1;
//                 }
//             }
//
//             if valid_flipps_from_past_window % 2 == nums[i] {
//                 if i + k > nums.len() {
//                     return -1;
//                 }
//                 valid_flipps_from_past_window += 1;
//                 flipped[i] = true;
//                 flip_count += 1;
//             }
//         }
//         flip_count
//     }
// }

// use std::collections::VecDeque;
//
// impl Solution {
//     pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
//         let k = k as usize;
//         let n = nums.len();
//         let mut flip_queue = VecDeque::new();
//         let mut flipped = 0;
//         let mut result = 0;
//
//         for i in 0..n {
//             if i >= k {
//                 flipped ^= flip_queue.pop_front().unwrap_or(0);
//             }
//
//             if flipped == nums[i] {
//                 if i + k > n {
//                     return -1;
//                 }
//                 flip_queue.push_back(1);
//                 flipped ^= 1;
//                 result += 1;
//             } else {
//                 flip_queue.push_back(0);
//             }
//         }
//
//         result
//     }
// }

impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k.try_into().unwrap();
        let mut current_flips = 0;
        let mut total_flips = 0;
        for i in 0..nums.len() {
            if i >= k && nums[i - k] == 2 {
                current_flips -= 1;
            }

            if (current_flips % 2) == nums[i] {
                if i + k > nums.len() {
                    return -1;
                }
                nums[i] = 2;
                current_flips += 1;
                total_flips += 1;
            }
        }
        total_flips
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
            3
        );
    }
}
