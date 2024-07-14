pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn case_2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn case_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }
}
