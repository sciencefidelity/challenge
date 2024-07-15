pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = nums[..usize::try_from(k).unwrap()].iter().sum::<i32>();
        let mut max = sum;

        for i in 0..nums.len() - usize::try_from(k).unwrap() {
            sum = sum - nums[i] + nums[i + usize::try_from(k).unwrap()];
            max = max.max(sum);
        }
        f64::from(max) / f64::from(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.0);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::find_max_average(vec![0, 4, 0, 3, 2], 1), 4.0);
    }
}
