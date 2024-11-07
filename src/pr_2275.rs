pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut max_count = 0;
        for i in 0..24 {
            let mut count = 0;
            for n in &candidates {
                if (n & (1 << i)) != 0 {
                    count += 1;
                }
            }
            max_count = max_count.max(count);
        }
        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]),
            4
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::largest_combination(vec![8, 8]), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::largest_combination(vec![71, 17, 16, 62, 12, 24, 14]),
            4
        );
    }
}
