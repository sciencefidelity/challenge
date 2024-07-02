pub struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut cur_max, mut cur_min) = (0, 0);
        let (mut max_sum, mut min_sum) = (nums[0], nums[0]);
        let mut total_sum = 0;
        for num in nums {
            cur_max = cur_max.max(0) + num;
            max_sum = max_sum.max(cur_max);

            cur_min = cur_min.min(0) + num;
            min_sum = min_sum.min(cur_min);

            total_sum += num;
        }
        if total_sum == min_sum {
            return max_sum;
        }
        max_sum.max(total_sum - min_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
    }
}
