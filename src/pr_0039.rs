use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combination_sum = CombinationSum::new(candidates, target);
        combination_sum.backtrack(&mut Vec::new(), target, 0);
        combination_sum.output
    }
}

struct CombinationSum {
    candidates: Vec<i32>,
    target: i32,
    output: Vec<Vec<i32>>,
}

impl CombinationSum {
    pub fn new(candidates: Vec<i32>, target: i32) -> Self {
        Self {
            candidates,
            target,
            output: Vec::new(),
        }
    }

    fn backtrack(&mut self, cur: &mut Vec<i32>, remain: i32, start: usize) {
        match remain.cmp(&0) {
            Ordering::Less => {}
            Ordering::Equal => self.output.push(cur.clone()),
            Ordering::Greater => {
                for i in start..self.candidates.len() {
                    cur.push(self.candidates[i]);
                    self.backtrack(cur, remain - self.candidates[i], i);
                    cur.pop();
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
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]],
            Solution::combination_sum(vec![2, 3, 6, 7], 7)
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            Solution::combination_sum(vec![2, 3, 5], 8)
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::combination_sum(vec![2], 1)
        );
    }
}
