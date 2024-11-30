use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adj = HashMap::<i32, Vec<i32>>::new();
        let mut deg = HashMap::<i32, i32>::new();
        for v in &pairs {
            adj.entry(v[1]).or_default().push(v[0]);
            adj.entry(v[0]).or_default();
            *deg.entry(v[1]).or_insert(0) += 1;
            *deg.entry(v[0]).or_insert(0) -= 1;
        }
        let last = *deg
            .iter()
            .find(|&(_, &x)| x == 1)
            .or_else(|| deg.iter().next())
            .unwrap()
            .0;
        let mut ans = Vec::<Vec<i32>>::with_capacity(pairs.len());
        Self::arrange(last, &mut ans, &mut adj);
        ans
    }

    fn arrange(curr: i32, ans: &mut Vec<Vec<i32>>, adj: &mut HashMap<i32, Vec<i32>>) {
        while let Some(prev) = adj.get_mut(&curr).unwrap().pop() {
            Self::arrange(prev, ans, adj);
            ans.push(vec![prev, curr]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::valid_arrangement(arr![[5, 1], [4, 5], [11, 9], [9, 4]]),
            arr![[11, 9], [9, 4], [4, 5], [5, 1]]
        );
    }

    #[test]
    fn case_2() {
        let expected = Solution::valid_arrangement(arr![[1, 3], [3, 2], [2, 1]]);
        assert!(
            expected == arr![[1, 3], [3, 2], [2, 1]]
                || expected == arr![[2, 1], [1, 3], [3, 2]]
                || expected == arr![[3, 2], [2, 1], [1, 3]]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::valid_arrangement(arr![[1, 2], [1, 3], [2, 1]]),
            arr![[1, 2], [2, 1], [1, 3]]
        );
    }
}
