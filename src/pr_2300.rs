pub struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n = potions.len();
        potions.sort_unstable();
        spells
            .into_iter()
            .map(i64::from)
            .map(|spell| potions.partition_point(|&potion| i64::from(potion) * spell < success))
            .map(|i| i32::try_from(n - i).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
