use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut freq = HashMap::with_capacity(nums.len());
        for num in &nums {
            freq.entry(*num).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        nums.sort_unstable_by_key(|num| (freq.get(num).unwrap(), -num));
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            vec![3, 1, 1, 2, 2, 2],
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            vec![1, 3, 3, 2, 2],
            Solution::frequency_sort(vec![2, 3, 1, 3, 2])
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1],
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])
        );
    }
}
