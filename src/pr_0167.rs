pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut high = match numbers.binary_search(&(target - numbers[0])) {
            Ok(i) => return vec![1, i as i32 + 1],
            Err(i) => i - 1,
        };
        let mut low = if high == numbers.len() - 1 {
            match numbers[..high].binary_search(&(target - numbers[high])) {
                Ok(i) => return vec![i as i32 + 1, high as i32 + 1],
                Err(i) => i,
            }
        } else {
            1
        };
        while low < high {
            match (numbers[low] + numbers[high]).cmp(&target) {
                Ordering::Equal => return vec![low as i32 + 1, high as i32 + 1],
                Ordering::Less => low += 1,
                Ordering::Greater => high -= 1,
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
