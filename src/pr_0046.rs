pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output = Vec::new();
        Self::backtrack(&nums, &mut output, &mut Vec::new());
        output
    }

    fn backtrack(nums: &[i32], output: &mut Vec<Vec<i32>>, comb: &mut Vec<i32>) {
        if nums.len() == comb.len() {
            output.push(comb.clone());
            return;
        }
        for i in 0..nums.len() {
            if comb.contains(&nums[i]) {
                continue;
            }
            comb.push(nums[i]);
            Self::backtrack(nums, output, comb);
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
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            Solution::permute(vec![1, 2, 3])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(vec![vec![0, 1], vec![1, 0],], Solution::permute(vec![0, 1]));
    }

    #[test]
    fn case_3() {
        assert_eq!(vec![vec![1]], Solution::permute(vec![1]));
    }
}
