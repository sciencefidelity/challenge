pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let k = usize::try_from(k).unwrap();
        let mut output = Vec::new();
        Self::backtrack(&mut output, &mut Vec::new(), k, 1, n);
        output
    }

    fn backtrack(output: &mut Vec<Vec<i32>>, comb: &mut Vec<i32>, k: usize, start: i32, n: i32) {
        if comb.len() > k {
            return;
        }
        if comb.len() == k && n == 0 {
            output.push(comb.clone());
            return;
        }
        let mut i = start;
        while i <= n && i <= 9 {
            comb.push(i);
            Self::backtrack(output, comb, k, i + 1, n - i);
            comb.pop();
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(vec![vec![1, 2, 4]], Solution::combination_sum3(3, 7));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]],
            Solution::combination_sum3(3, 9)
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::combination_sum3(4, 1));
    }
}
