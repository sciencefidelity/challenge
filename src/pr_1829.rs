pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor_product = 0;
        for n in &nums {
            xor_product ^= n;
        }
        let mut ans = vec![0; nums.len()];
        let mask = (1 << maximum_bit) - 1;
        for (i, n) in ans.iter_mut().enumerate() {
            *n = xor_product ^ mask;
            xor_product ^= nums[nums.len() - 1 - i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::get_maximum_xor(vec![0, 1, 1, 3], 2),
            vec![0, 3, 2, 3]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::get_maximum_xor(vec![2, 3, 4, 7], 3),
            vec![5, 2, 6, 5]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3),
            vec![4, 3, 6, 4, 6, 7]
        );
    }
}
