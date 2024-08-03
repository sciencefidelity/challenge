pub struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let sum: i32 = nums.iter().sum();
        nums.into_iter()
            .scan(sum, |rs, x| {
                let ls = sum - *rs;
                let d = (ls - *rs + x).abs();
                *rs -= x;
                Some(d)
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
            vec![15, 1, 11, 22],
            Solution::left_right_difference(vec![10, 4, 8, 3])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(vec![0], Solution::left_right_difference(vec![0]));
    }
}
