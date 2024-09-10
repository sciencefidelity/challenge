pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let (mut res, n) = (Vec::new(), nums.len());
        Self::backtrack(&mut res, &nums, &mut Vec::new(), &mut vec![false; n]);
        res
    }

    fn backtrack(
        res: &mut Vec<Vec<i32>>,
        nums: &[i32],
        comb: &mut Vec<i32>,
        visited: &mut Vec<bool>,
    ) {
        if comb.len() == nums.len() {
            res.push(comb.clone());
            return;
        }
        for (idx, &num) in nums.iter().enumerate() {
            if visited[idx] || (idx > 0 && num == nums[idx - 1] && !visited[idx - 1]) {
                continue;
            }
            visited[idx] = true;
            comb.push(num);
            Self::backtrack(res, nums, comb, visited);
            comb.pop();
            visited[idx] = false;
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
            Solution::permute_unique(vec![1, 1, 2]),
            arr![[1, 1, 2], [1, 2, 1], [2, 1, 1]]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            arr![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::permute_unique(vec![3, 3, 0, 3]),
            arr![[0, 3, 3, 3], [3, 0, 3, 3], [3, 3, 0, 3], [3, 3, 3, 0]]
        );
    }
}
