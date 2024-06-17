pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1_set: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
        let nums2_set: HashSet<i32> = HashSet::from_iter(nums2.into_iter());

        vec![
            nums1_set
                .difference(&nums2_set)
                .into_iter()
                .cloned()
                .collect(),
            nums2_set
                .difference(&nums1_set)
                .into_iter()
                .cloned()
                .collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]),
            vec![vec![1, 3], vec![4, 6]]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec![vec![3], vec![]]
        );
    }
}
