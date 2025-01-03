pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut left_sum = 0_i64;
        let mut right_sum = nums.iter().map(|&x| i64::from(x)).sum::<i64>();
        let n = nums.len();
        let mut count = 0;
        for num in nums.into_iter().take(n - 1) {
            let num = i64::from(num);
            left_sum += num;
            right_sum -= num;
            if left_sum >= right_sum {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::ways_to_split_array(vec![10, 4, -8, 7]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::ways_to_split_array(vec![2, 3, 1, 0]), 2);
    }
}
