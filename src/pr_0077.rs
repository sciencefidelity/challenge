pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let (k, mut output) = (usize::try_from(k).unwrap(), Vec::new());
        Self::backtrack(&mut output, &mut Vec::new(), k, 1, n);
        output
    }

    fn backtrack(output: &mut Vec<Vec<i32>>, comb: &mut Vec<i32>, k: usize, start: i32, n: i32) {
        if comb.len() == k {
            output.push(comb.clone());
            return;
        }
        for i in start..=n {
            comb.push(i);
            Self::backtrack(output, comb, k, i + 1, n);
            comb.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ],
            Solution::combine(4, 2)
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(vec![vec![1]], Solution::combine(1, 1));
    }
}
