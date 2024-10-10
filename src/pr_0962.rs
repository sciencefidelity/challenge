pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let (n, mut stack) = (nums.len(), Vec::new());
        for i in 0..n {
            if stack.is_empty() || nums[stack[stack.len() - 1]] > nums[i] {
                stack.push(i);
            }
        }
        let mut max_width = 0;
        for j in (0..n).rev() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] <= nums[j] {
                max_width = max_width.max(j - stack[stack.len() - 1]);
                stack.pop();
            }
        }
        i32::try_from(max_width).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
            7
        );
    }
}
