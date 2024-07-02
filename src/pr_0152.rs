pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max, mut cur_max, mut cur_min) =
            (nums[0] as i128, nums[0] as i128, nums[0] as i128);
        for &num in nums.iter().skip(1) {
            let num = num as i128;
            let temp1 = num * cur_max;
            let temp2 = num * cur_min;
            cur_max = temp1.max(temp2).max(num);
            cur_min = temp1.min(temp2).min(num);
            max = max.max(cur_max);
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::max_product(vec![
                0, 10, 10, 10, 10, 10, 10, 10, 10, 10, -10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0
            ]),
            1000000000
        );
    }
}
