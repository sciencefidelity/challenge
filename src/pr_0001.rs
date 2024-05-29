pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&diff_index) = map.get(&diff) {
                return vec![diff_index, i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
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
