pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max = 0;
        while left < right {
            max = max.max(height[left].min(height[right]) * i32::try_from(right - left).unwrap());
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
