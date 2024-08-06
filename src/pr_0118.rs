pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = usize::try_from(num_rows).unwrap();
        let mut output = vec![vec![]; n];
        for i in 0..n {
            for j in 0..=i {
                let next = if j > 0 && j < i {
                    output[i - 1][j - 1] + output[i - 1][j]
                } else {
                    1
                };
                output[i].push(next);
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(expected, Solution::generate(5));
    }

    #[test]
    fn case_2() {
        let expected = vec![vec![1]];
        assert_eq!(expected, Solution::generate(1));
    }
}
