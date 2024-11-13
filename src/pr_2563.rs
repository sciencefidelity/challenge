#![allow(clippy::cast_possible_wrap)]
pub struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut ans = 0i64;
        for (idx, num) in nums.iter().copied().enumerate() {
            let l_bound = nums[idx + 1..].partition_point(|&n| n + num < lower);
            let u_bound = nums[idx + 1..].partition_point(|&n| n + num <= upper);
            ans += (u_bound - l_bound) as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
}
