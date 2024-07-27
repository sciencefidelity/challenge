use std::cmp::min;

pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg, clippy::needless_pass_by_value)]
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        const N: usize = 26;
        const A: usize = 97;
        let mut total_cost = 0;
        let mut min_cost = vec![vec![i64::MAX; N]; N];
        for (i, (o_char, c_char)) in original.into_iter().zip(changed.into_iter()).enumerate() {
            let start_char = o_char as usize - A;
            let end_char = c_char as usize - A;
            min_cost[start_char][end_char] = min(min_cost[start_char][end_char], cost[i].into());
        }
        for k in 0..N {
            for i in 0..N {
                for j in 0..N {
                    min_cost[i][j] = min(
                        min_cost[i][j],
                        min_cost[i][k].saturating_add(min_cost[k][j]),
                    );
                }
            }
        }
        for (s_byte, t_byte) in source.bytes().zip(target.bytes()) {
            if s_byte == t_byte {
                continue;
            }
            let source_char = usize::from(s_byte - b'a');
            let target_char = usize::from(t_byte - b'a');
            if min_cost[source_char][target_char] == i64::MAX {
                return -1;
            }
            total_cost += min_cost[source_char][target_char];
        }
        total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
        let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
        let cost = vec![2, 5, 5, 1, 2, 20];
        assert_eq!(
            28,
            Solution::minimum_cost(
                "abcd".to_owned(),
                "acbe".to_owned(),
                original,
                changed,
                cost
            ),
        );
    }

    #[test]
    fn case_2() {
        let original = vec!['a', 'c'];
        let changed = vec!['c', 'b'];
        let cost = vec![1, 2];
        assert_eq!(
            12,
            Solution::minimum_cost(
                "aaaa".to_owned(),
                "bbbb".to_owned(),
                original,
                changed,
                cost
            ),
        );
    }

    #[test]
    fn case_3() {
        let original = vec!['a'];
        let changed = vec!['e'];
        let cost = vec![1000];
        assert_eq!(
            -1,
            Solution::minimum_cost(
                "abcd".to_owned(),
                "abce".to_owned(),
                original,
                changed,
                cost
            ),
        );
    }
}
