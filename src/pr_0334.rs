pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut low = nums[0];
        let mut mid = i32::MAX;

        for num in nums {
            if num <= low {
                low = num;
            } else if num <= mid {
                mid = num;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn case_3() {
        assert!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }
}
