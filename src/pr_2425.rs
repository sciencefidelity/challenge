pub struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut xor1, mut xor2) = (0, 0);
        let (len1, len2) = (nums1.len(), nums2.len());
        if len2 % 2 != 0 {
            for num in nums1 {
                xor1 ^= num;
            }
        }
        if len1 % 2 != 0 {
            for num in nums2 {
                xor2 ^= num;
            }
        }
        xor1 ^ xor2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::xor_all_nums(vec![1, 2], vec![3, 4]), 0);
    }
}
