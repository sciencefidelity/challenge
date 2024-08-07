pub struct Solution;

// use std::collections::HashSet;
//
// impl Solution {
//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         let n = i32::try_from(nums.len()).unwrap();
//         let mut set = HashSet::with_capacity(nums.len());
//         for n in nums {
//             set.insert(n);
//         }
//         let mut i = 0;
//         while i < n {
//             if !set.contains(&i) {
//                 return i;
//             }
//             i += 1;
//         }
//         i
//     }
// }

// impl Solution {
//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         let n = i32::try_from(nums.len()).unwrap();
//         let mut full_sum = 0;
//         let current_sum: i32 = nums.iter().sum();
//         for i in 0..=n {
//             full_sum += i;
//         }
//         full_sum - current_sum
//     }
// }

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (1..=i32::try_from(nums.len()).unwrap())
            .zip(nums)
            .fold(0, |xor, (x, y)| xor ^ x ^ y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
    }

    #[test]
    fn case_2() {
        assert_eq!(2, Solution::missing_number(vec![0, 1]));
    }

    #[test]
    fn case_3() {
        assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
    }
}
