use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<&i32> = HashSet::from_iter(&nums1);
        let set2: HashSet<&i32> = HashSet::from_iter(&nums2);

        set1.intersection(&set2).map(|n| **n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }
}
