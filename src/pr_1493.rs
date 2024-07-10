pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut longest, mut left, mut deleted) = (0, 0, 0);
        for right in 0..nums.len() {
            if nums[right] == 0 {
                deleted += 1;
            }
            while deleted > 1 {
                if nums[left] == 0 {
                    deleted -= 1;
                }
                left += 1;
            }
            longest = longest.max(right - left);
        }
        longest.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::longest_subarray(vec![1, 1, 0, 0, 1, 1, 1, 0, 1]),
            4
        );
    }
}
