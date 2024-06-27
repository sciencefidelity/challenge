pub struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[1].contains(&edges[0][0]) {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
