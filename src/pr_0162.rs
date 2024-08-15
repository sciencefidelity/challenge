pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if nums.len() == 1 || nums[0] > nums[1] {
            return 0;
        }
        if nums[n - 1] > nums[n - 2] {
            return i32::try_from(n - 1).unwrap();
        }
        let (mut l, mut r) = (1, n - 2);
        while l <= r {
            let m = (l + r) / 2;
            if nums[m] > nums[m - 1] && nums[m] > nums[m + 1] {
                return i32::try_from(m).unwrap();
            } else if nums[m] < nums[m - 1] {
                r = m - 1;
            } else if nums[m] < nums[m + 1] {
                l = m + 1;
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
        assert_eq!(2, Solution::find_peak_element(vec![1, 2, 3, 1]));
    }

    #[test]
    fn case_2() {
        assert_eq!(5, Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]));
    }

    #[test]
    fn case_3() {
        assert_eq!(1, Solution::find_peak_element(vec![1, 3, 2, 1]));
    }

    #[test]
    fn case_4() {
        assert_eq!(0, Solution::find_peak_element(vec![4, 3, 2, 1]));
    }
}
