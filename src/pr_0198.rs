pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(pp, p), curr| (p, p.max(curr + pp)))
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn case_2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
