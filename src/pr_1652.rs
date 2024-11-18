#![allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![0; code.len()];
        let n = code.len() as i32;
        if k == 0 {
            return ans;
        }
        let (mut start, mut end, mut sum) = (1, k, 0);
        if k < 0 {
            start = n - k.abs();
            end = n - 1;
        }
        for i in start..=end {
            sum += code[i as usize];
        }
        for i in 0..n {
            ans[i as usize] = sum;
            sum -= code[(start % n) as usize];
            sum += code[((end + 1) % n) as usize];
            start += 1;
            end += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
