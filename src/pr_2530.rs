use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut result = 0;
        let mut pq = BinaryHeap::from(nums);
        for _ in 0..k {
            if let Some(max_element) = pq.pop() {
                result += i64::from(max_element);
                pq.push((max_element + 2) / 3);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }
}
