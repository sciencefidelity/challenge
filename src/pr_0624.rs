pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut result = i32::MIN;
        let (mut min, mut max) = (arrays[0][0], arrays[0][arrays[0].len() - 1]);
        for array in arrays.into_iter().skip(1) {
            result = result.max((array[0] - max).abs());
            result = result.max((array[array.len() - 1] - min).abs());
            (min, max) = (min.min(array[0]), max.max(array[array.len() - 1]));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            4,
            Solution::max_distance(arr![[1, 2, 3], [4, 5], [1, 2, 3]])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(0, Solution::max_distance(arr![[1], [1]]));
    }
}
