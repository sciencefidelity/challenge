use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 1, 1), |(x, cnt, res), n| match n.cmp(&x) {
                Ordering::Greater => (n, 1, 1),
                Ordering::Equal => (x, cnt + 1, res.max(cnt + 1)),
                Ordering::Less => (x, 0, res),
            })
            .2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 4]), 1);
    }
}
