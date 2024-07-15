pub struct Solution;

// use std::collections::HashMap;
//
// impl Solution {
//     pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
//         let mut curr_sum = 0;
//         let mut subarrays = 0;
//         let mut prefix_sum = HashMap::from([(curr_sum, 1)]);
//         for num in nums.iter() {
//             curr_sum += num % 2;
//             if let Some(&val) = prefix_sum.get(&(curr_sum - k)) {
//                 subarrays += val;
//             }
//             *prefix_sum.entry(curr_sum).or_insert(0) += 1;
//         }
//         subarrays
//     }
// }

// use std::collections::VecDeque;
//
// impl Solution {
//     pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
//         let mut odd_indicies = VecDeque::new();
//         let mut subarrays = 0;
//         let mut last_popped = -1;
//         #[warn(unused_assignments)]
//         let mut initial_gap = -1;
//         for i in 0..nums.len() {
//             if nums[i] % 2 == 1 {
//                 odd_indicies.push_back(i as i32);
//             }
//             if odd_indicies.len() > k as usize {
//                 last_popped = odd_indicies.pop_front().unwrap();
//             }
//             if odd_indicies.len() == k as usize {
//                 initial_gap = odd_indicies.front().unwrap() - last_popped;
//                 subarrays += initial_gap;
//             }
//         }
//         subarrays
//     }
// }

// impl Solution {
//     pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
//         let mut subarrays = 0;
//         let mut initial_gap = 0;
//         let mut qsize = 0;
//         let mut start = 0;
//         for end in 0..nums.len() {
//             if nums[end] % 2 == 1 {
//                 qsize += 1;
//             }
//             if qsize == k {
//                 initial_gap = 0;
//                 while qsize == k {
//                     qsize -= nums[start] % 2;
//                     initial_gap += 1;
//                     start += 1;
//                 }
//             }
//             subarrays += initial_gap;
//         }
//         subarrays
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        Self::at_most(&nums, k) - Self::at_most(&nums, k - 1)
    }

    fn at_most(nums: &[i32], k: i32) -> i32 {
        let mut window_size = 0;
        let mut subarrays = 0;
        let mut start = 0;
        for end in 0..nums.len() {
            window_size += nums[end] % 2;
            while window_size > k {
                window_size -= nums[start] % 2;
                start += 1;
            }
            subarrays += i32::try_from(end - start + 1).unwrap();
        }
        subarrays
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
