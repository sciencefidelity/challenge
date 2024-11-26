#![allow(clippy::cast_sign_loss)]
pub struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut indegree = vec![0; n as usize];
        for edge in edges {
            indegree[edge[1] as usize] += 1;
        }
        let (mut champ, mut champ_count) = (-1, 0);
        for i in 0..n {
            if indegree[i as usize] == 0 {
                champ_count += 1;
                champ = i;
            }
        }
        if champ_count > 1 {
            -1
        } else {
            champ
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_champion(3, arr![[0, 1], [1, 2]]), 0);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_champion(4, arr![[0, 2], [1, 3], [1, 2]]), -1);
    }
}
