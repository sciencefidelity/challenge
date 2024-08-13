pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut highest_index = 0;
        let mut current_index = 0;

        while current_index < nums.len() - 1 {
            if nums[current_index] == 0 && highest_index <= current_index {
                return false;
            }
            highest_index =
                highest_index.max(current_index + usize::try_from(nums[current_index]).unwrap());
            current_index += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn case_3() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn case_4() {
        assert!(!Solution::can_jump(vec![0, 2, 3]));
    }
}
