pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut low = 0;
        let mut high = 0;
        while high < nums.len() {
            if nums[high] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[low] == 0 {
                    k += 1;
                }
                low += 1;
            }
            high += 1;
        }
        (high - low) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
