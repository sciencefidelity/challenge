pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        assert_eq!(nums, vec![0, 1, 0, 3, 12]);
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0];
        assert_eq!(nums, vec![0]);
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn case_3() {
        let mut nums = vec![0, 1];
        assert_eq!(nums, vec![0, 1]);
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0]);
    }
}
