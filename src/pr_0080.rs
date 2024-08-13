pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut low = 0;
        for high in 0..nums.len() {
            if low < 2 || nums[low - 2] < nums[high] {
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
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k: usize = 5;
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            i32::try_from(k).unwrap()
        );
        assert_eq!(nums[..k], vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k: usize = 7;
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            i32::try_from(k).unwrap()
        );
        assert_eq!(nums[..k], vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
