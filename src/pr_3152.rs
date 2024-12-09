#![allow(clippy::needless_pass_by_value, clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut prefix_count = vec![0; n];
        for i in 0..n - 1 {
            prefix_count[i + 1] = prefix_count[i];
            if (nums[i] + nums[i + 1]) % 2 == 1 {
                prefix_count[i + 1] += 1;
            }
        }
        let mut answers = Vec::with_capacity(queries.len());
        for query in queries {
            if let [from, to] = &query[..2] {
                let diff = prefix_count[*to as usize] - prefix_count[*from as usize];
                answers.push(diff == *to - *from);
            }
        }
        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::is_array_special(vec![3, 4, 1, 2, 6], arr![[0, 4]]),
            vec![false]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::is_array_special(vec![4, 3, 1, 6], arr![[0, 2], [2, 3]]),
            vec![false, true]
        );
    }
}
