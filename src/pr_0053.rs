pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut max_so_far, mut max_ending_here) = (nums[0], nums[0]);
        for num in nums.into_iter().skip(1) {
            max_ending_here = num.max(max_ending_here + num);
            max_so_far = max_so_far.max(max_ending_here);
        }
        max_so_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
