pub struct Solution;

impl Solution {
    pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
        let mut min = i64::MAX;
        let mut points: i64 = 0;
        for i in 0..enemy_energies.len() {
            min = min.min(enemy_energies[i] as i64);
            points += enemy_energies[i] as i64;
        }
        if i64::from(current_energy) < min {
            return 0;
        }
        points -= min;
        (current_energy as i64 / min + (current_energy as i64 % min + points) / min).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::maximum_points(vec![3, 2, 2], 2), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::maximum_points(vec![2], 10), 5);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::maximum_points(vec![1, 2, 3], 0), 0);
    }
}
