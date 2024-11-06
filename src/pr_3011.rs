pub struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let (mut curr_min, mut curr_max) = (nums[0], nums[0]);
        let mut last_max = 0;
        let mut curr_ones = nums[0].count_ones();
        for n in nums.into_iter().skip(1) {
            let ones = n.count_ones();
            if ones == curr_ones {
                (curr_min, curr_max) = (curr_min.min(n), curr_max.max(n));
            } else {
                (last_max, curr_ones) = (curr_max, ones);
                (curr_min, curr_max) = (n, n);
            }
            if last_max > curr_min {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]));
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::can_sort_array(vec![3, 16, 8, 4, 2]));
    }
}
