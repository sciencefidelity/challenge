pub struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut tups: Vec<(usize, i32)> = nums.iter().copied().enumerate().collect();
        tups.sort_unstable_by_key(|k| k.1);
        for chunk in tups.chunk_by(|a, b| b.1 - a.1 <= limit) {
            let mut indices = chunk.to_vec();
            indices.sort_unstable();
            for i in 0..indices.len() {
                let idx = indices[i].0;
                nums[idx] = chunk[i].1;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
            vec![1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 2),
            vec![1, 6, 7, 18, 1, 2]
        );
    }
}
