pub struct Solution;

// use std::{cmp::Reverse, collections::BinaryHeap};
//
// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
//         let (n, left, right) = (
//             usize::try_from(n).unwrap(),
//             usize::try_from(left).unwrap(),
//             usize::try_from(right).unwrap(),
//         );
//         let mut queue = BinaryHeap::with_capacity(n * (n + 1) / 2);
//         for i in 0..n {
//             let mut running_sum = 0;
//             for num in nums.iter().take(n).skip(i) {
//                 running_sum += num;
//                 queue.push(Reverse(running_sum));
//             }
//         }
//         let mut sum = 0;
//         for i in 0..right {
//             let n = queue.pop().unwrap().0;
//             if i >= left - 1 {
//                 sum += n;
//             }
//         }
//         sum % 1_000_000_007
//     }
// }

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let m = 1_000_000_007;
        let result =
            Self::sum_of_first_k(&nums, n, right) - Self::sum_of_first_k(&nums, n, left - 1);
        i32::try_from((result + m) % m).unwrap()
    }

    fn count_and_sum(nums: &Vec<i32>, n: i32, target: i32) -> (i32, i64) {
        let mut count = 0;
        let (mut current_sum, mut total_sum, mut window_sum): (i64, i64, i64) = (0, 0, 0);
        let mut i = 0;
        for j in 0..n {
            current_sum += i64::from(nums[j as usize]);
            window_sum += i64::from(nums[j as usize] * (j - i + 1));
            while current_sum > target.into() {
                window_sum -= current_sum;
                current_sum -= i64::from(nums[i as usize]);
                i += 1;
            }
            count += j - i + 1;
            total_sum += window_sum;
        }
        (count, total_sum)
    }

    fn sum_of_first_k(nums: &Vec<i32>, n: i32, k: i32) -> i64 {
        let min_sum = nums.iter().min().unwrap();
        let max_sum = nums.iter().sum::<i32>();
        let mut left = *min_sum;
        let mut right = max_sum;
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::count_and_sum(&nums, n, mid).0 >= k {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        let (count, total_sum) = Self::count_and_sum(&nums, n, left);
        total_sum - i64::from(left) * i64::from(count - k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(13, Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5));
    }

    #[test]
    fn case_2() {
        assert_eq!(6, Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4));
    }

    #[test]
    fn case_3() {
        assert_eq!(50, Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10));
    }
}
