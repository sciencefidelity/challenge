pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut map = HashMap::from([(0, 1)]);
        for num in nums {
            sum += num;
            if let Some(&val) = map.get(&(sum - k)) {
                count += val;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
