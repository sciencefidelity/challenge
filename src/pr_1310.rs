pub struct Solution;

impl Solution {
    pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cur = 0;
        for a in &mut arr {
            cur ^= *a;
            *a = cur;
        }
        queries
            .into_iter()
            .map(|q| {
                if q[0] > 0 {
                    arr[usize::try_from(q[0] - 1).unwrap()] ^ arr[usize::try_from(q[1]).unwrap()]
                } else {
                    arr[usize::try_from(q[1]).unwrap()]
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::xor_queries(vec![1, 3, 4, 8], arr![[0, 1], [1, 2], [0, 3], [3, 3]]),
            vec![2, 7, 14, 8]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::xor_queries(vec![4, 8, 2, 10], arr![[2, 3], [1, 3], [0, 0], [0, 3]]),
            vec![8, 0, 4, 4]
        );
    }
}
