pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.binary_search_by(|num| num[0].cmp(&target)) {
            Ok(_) => true,
            Err(0) => false,
            Err(index) => matrix[index - 1].binary_search(&target).is_ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        let matrix = arr![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn case_2() {
        let matrix = arr![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]];
        assert!(!Solution::search_matrix(matrix, 13));
    }
}
