pub struct Solution;

impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut min_increments = 0;
        let max = nums.iter().max().unwrap();
        let mut frequency_count = vec![0; nums.len() + usize::try_from(*max).unwrap()];

        for val in nums {
            frequency_count[usize::try_from(val).unwrap()] += 1;
        }

        for i in 0..frequency_count.len() {
            if frequency_count[i] <= 1 {
                continue;
            }
            let duplicates = frequency_count[i] - 1;
            frequency_count[i + 1] += duplicates;
            frequency_count[i] = 1;
            min_increments += duplicates;
        }
        min_increments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]),
            6
        );
    }
}
