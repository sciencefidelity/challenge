pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        #[allow(clippy::needless_pass_by_value)]
        let mut low = 1;
        for high in 1..nums.len() {
            if nums[low - 1] < nums[high] {
                nums[low] = nums[high];
                low += 1;
            }
        }
        low.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![1, 1, 2];
        let k: usize = 2;
        assert_eq!(Solution::remove_duplicates(&mut nums), k as i32);
        assert_eq!(nums[..k], vec![1, 2]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k: usize = 5;
        assert_eq!(Solution::remove_duplicates(&mut nums), k as i32);
        assert_eq!(nums[..k], vec![0, 1, 2, 3, 4]);
    }
}
