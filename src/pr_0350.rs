pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut intersection = vec![];
        for num in nums1 {
            *map.entry(num).or_insert(0) += 1;
        }
        for num in nums2 {
            match map.get_mut(&num) {
                Some(val) if *val > 0 => *val -= 1,
                _ => continue,
            };
            intersection.push(num);
        }
        intersection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
    }

    #[test]
    fn case_2() {
        let intersection = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(&4));
        assert!(intersection.contains(&9));
    }
}
