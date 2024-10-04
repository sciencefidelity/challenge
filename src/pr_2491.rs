pub struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let (n, mut total_skill, mut frequency) = (skill.len(), 0, vec![0; 1001]);
        for player in &skill {
            total_skill += player;
            frequency[usize::try_from(*player).unwrap()] += 1;
        }
        if total_skill % (i32::try_from(n).unwrap() / 2) != 0 {
            return -1;
        }
        let (target_skill, mut chemistry) = (total_skill / i32::try_from(n / 2).unwrap(), 0);
        for player in skill {
            let partner = target_skill - player;
            if frequency[usize::try_from(partner).unwrap()] == 0 {
                return -1;
            }
            chemistry += i64::from(player) * i64::from(partner);
            frequency[usize::try_from(partner).unwrap()] -= 1;
        }
        chemistry / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::divide_players(vec![3, 4]), 12);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::divide_players(vec![1, 1, 2, 3]), -1);
    }
}
