pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let (mut left, mut right) = (1, *nums.iter().max().unwrap());
        while left < right {
            let middle = left + (right - left) / 2;
            let ops = nums.iter().map(|&n| (n - 1) / middle).sum::<i32>();
            if ops > max_operations {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimum_size(vec![9], 2), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimum_size(vec![2, 4, 8, 2], 4), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::minimum_size(vec![7, 17], 2), 7);
    }
}
