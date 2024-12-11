#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let max_value = nums.iter().max().unwrap();
        let mut count = vec![0; (max_value + 1) as usize];
        for num in &nums {
            count[0.max(num - k) as usize] += 1;
            if num + k < *max_value {
                count[(num + k + 1) as usize] -= 1;
            }
        }
        let mut max_beauty = 0;
        let mut current_sum = 0;
        for val in count {
            current_sum += val;
            max_beauty = max_beauty.max(current_sum);
        }
        max_beauty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_beauty(vec![1, 1, 1, 1], 10), 4);
    }
}
