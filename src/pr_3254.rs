pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value, clippy::cast_sign_loss)]
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.windows(k as usize)
            .map(|sub| {
                if sub.windows(2).all(|w| w[1] == w[0] + 1) {
                    sub[sub.len() - 1]
                } else {
                    -1
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::results_array(vec![2, 2, 2, 2, 2], 4),
            vec![-1, -1]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2),
            vec![-1, 3, -1, 3, -1]
        );
    }
}
