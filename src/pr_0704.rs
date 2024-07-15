pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0_usize;
        let mut high = nums.len();
        let mut mid: usize;
        while low < high {
            mid = (low + high) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
                Ordering::Equal => return mid.try_into().unwrap_or(-1),
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
