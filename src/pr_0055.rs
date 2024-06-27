pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut highest_index = 0;
        let mut current_index = 0;

        while current_index < nums.len() - 1 {
            if nums[current_index] == 0 && highest_index <= current_index {
                return false;
            }
            highest_index = highest_index.max(current_index + nums[current_index] as usize);
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
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::can_jump(vec![0]), true);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::can_jump(vec![0, 2, 3]), false);
    }
}