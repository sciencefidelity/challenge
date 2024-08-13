use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combination_sum = CombinationSum::new(candidates, target);
        combination_sum.backtrack(0, target);
        combination_sum.output
    }
}

struct CombinationSum {
    candidates: Vec<i32>,
    target: i32,
    output: Vec<Vec<i32>>,
    combination: Vec<i32>,
}

impl CombinationSum {
    pub fn new(mut candidates: Vec<i32>, target: i32) -> Self {
        candidates.sort_unstable();
        Self {
            candidates,
            target,
            output: Vec::new(),
            combination: Vec::new(),
        }
    }

    pub fn backtrack(&mut self, start: usize, rem: i32) {
        match rem.cmp(&0) {
            Ordering::Less => {}
            Ordering::Equal => self.output.push(self.combination.clone()),
            Ordering::Greater => {
                let mut i = start;
                while i < self.candidates.len() && self.target >= self.candidates[i] {
                    if i == start || self.candidates[i] != self.candidates[i - 1] {
                        self.combination.push(self.candidates[i]);
                        self.backtrack(i + 1, rem - self.candidates[i]);
                        self.combination.pop();
                    }
                    i += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        assert_eq!(
            expected,
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        );
    }

    #[test]
    fn case_2() {
        let expected = vec![vec![1, 2, 2], vec![5]];
        assert_eq!(expected, Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5));
    }
}
