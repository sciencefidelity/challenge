pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap();
        while left < right {
            let middle = (left + right) / 2;
            if n >= quantities
                .iter()
                .map(|q| (q + middle - 1) / middle)
                .sum::<i32>()
            {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::minimized_maximum(7, vec![15, 10, 10]), 5);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::minimized_maximum(1, vec![1000]), 1000);
    }
}
