pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if i32::try_from(original.len()).unwrap() == m * n {
            original
                .chunks(usize::try_from(n).unwrap())
                .map(<[i32]>::to_vec)
                .collect()
        } else {
            Vec::new()
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
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            arr![[1, 2], [3, 4]]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3], 1, 3),
            arr![[1, 2, 3]]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2], 1, 1),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3),
            arr![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        );
    }
}
