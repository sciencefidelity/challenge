pub struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let num_rolls = i32::try_from(rolls.len()).unwrap() + n;
        let sum: i32 = rolls.into_iter().sum();
        let remaining_sum = mean * num_rolls - sum;
        if remaining_sum > 6 * n || remaining_sum < n {
            return vec![];
        }
        let distribute_mean = remaining_sum / n;
        let mod_rem = remaining_sum % n;
        let mut result = vec![distribute_mean; usize::try_from(n).unwrap()];
        for r in result.iter_mut().take(usize::try_from(mod_rem).unwrap()) {
            *r += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::missing_rolls(vec![1, 5, 6], 3, 4),
            vec![3, 2, 2, 2]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4), vec![]);
    }
}
