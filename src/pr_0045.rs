pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut max_distance, mut current_pos, mut jumps) = (0, 0, 0);
        for (i, num) in nums.into_iter().enumerate() {
            if i == n - 1 {
                break;
            }
            max_distance = max_distance.max(i + usize::try_from(num).unwrap());
            if i == current_pos {
                jumps += 1;
                current_pos = max_distance;
            }
        }
        jumps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::jump(vec![0]), 0);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::jump(vec![1]), 0);
    }

    #[test]
    fn case_5() {
        assert_eq!(
            Solution::jump(vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3]),
            2
        );
    }
}
