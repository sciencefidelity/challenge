pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum_left = vec![0];
        let mut sum_right = vec![0];
        for (i, num) in nums.iter().enumerate() {
            sum_left.push(sum_left[i] + num);
            sum_right.push(sum_right[i] + nums[n - i - 1]);
        }
        for (i, _) in nums.into_iter().enumerate() {
            if sum_left[i] == sum_right[n - i - 1] {
                return i32::try_from(i).unwrap();
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }
}
