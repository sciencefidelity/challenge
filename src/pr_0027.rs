pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![3, 2, 2, 3];
        let k: usize = 2;
        assert_eq!(Solution::remove_element(&mut nums, 3), k as i32);
        assert_eq!(nums[..k], vec![2, 2]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k: usize = 5;
        assert_eq!(Solution::remove_element(&mut nums, 2), k as i32);
        assert_eq!(nums[..k], vec![0, 1, 3, 0, 4]);
    }
}
