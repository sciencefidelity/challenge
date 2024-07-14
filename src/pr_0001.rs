pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        let mut result = Vec::with_capacity(2);
        nums.into_iter().enumerate().for_each(|(i, num)| {
            let diff = target - num;
            if let Some(&i_diff) = map.get(&diff) {
                return result = vec![i_diff, i32::try_from(i).unwrap()];
            }
            map.insert(num, i32::try_from(i).unwrap());
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(Solution::two_sum(nums, 9), [0, 1]);
    }

    #[test]
    fn case_2() {
        let nums = vec![3, 2, 4];
        assert_eq!(Solution::two_sum(nums, 6), [1, 2]);
    }

    #[test]
    fn case_3() {
        let nums = vec![3, 3];
        assert_eq!(Solution::two_sum(nums, 6), [0, 1]);
    }
}
