pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum = nums[..k].iter().sum::<i32>();
        let mut max = sum;

        for i in 0..nums.len() - k {
            sum = sum - nums[i] + nums[i + k];
            max = max.max(sum);
        }
        max as f64 / k as f64
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
