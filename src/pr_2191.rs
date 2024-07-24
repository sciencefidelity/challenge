pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_cached_key(|n| Self::map_numbers(&mapping, usize::try_from(*n).unwrap()));
        nums
    }

    const fn map_numbers(mapping: &[i32], mut num: usize) -> i32 {
        if num == 0 {
            return mapping[0];
        }
        let mut output = 0;
        let mut multiplier = 1;
        while num > 0 {
            output += multiplier * mapping[num % 10];
            num /= 10;
            multiplier *= 10;
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mapping = vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6];
        assert_eq!(
            vec![338, 38, 991],
            Solution::sort_jumbled(mapping, vec![991, 338, 38])
        );
    }

    #[test]
    fn case_2() {
        let mapping = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            vec![123, 456, 789],
            Solution::sort_jumbled(mapping, vec![789, 456, 123])
        );
    }

    #[test]
    fn case_3() {
        let mapping = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            Solution::sort_jumbled(mapping, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
    }
}
