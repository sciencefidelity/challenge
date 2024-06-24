pub struct Solution;

impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable();
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut min_average: f64 = f64::MAX;
        while left < right {
            min_average = min_average.min((nums[left] + nums[right]) as f64 / 2.0);
            left += 1;
            right -= 1;
        }
        min_average
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]),
            5.5
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
}
