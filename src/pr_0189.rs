pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let size = nums.len();
        if size <= 1 {
            return;
        }
        let k = k as usize % size;
        if k == 0 {
            return;
        }
        Self::reverse(nums, 0, nums.len() - 1);
        Self::reverse(nums, 0, k - 1);
        Self::reverse(nums, k, size - 1);
    }

    fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn case_3() {
        let mut nums = vec![-1];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![-1]);
    }
}
