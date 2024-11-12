pub struct Solution;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable();
        items.dedup_by(|right, left| right[1] <= left[1]);
        queries
            .into_iter()
            .map(|q| match items.partition_point(|it| it[0] <= q) {
                0 => 0,
                i => items[i - 1][1],
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
        let items = arr![[1, 2], [3, 2], [2, 4], [5, 6], [3, 5]];
        let queries = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![2, 4, 5, 5, 6, 6];
        assert_eq!(Solution::maximum_beauty(items, queries), expected);
    }

    #[test]
    fn case_2() {
        let items = arr![[1, 2], [1, 2], [1, 3], [1, 4]];
        let queries = vec![1];
        let expected = vec![4];
        assert_eq!(Solution::maximum_beauty(items, queries), expected);
    }

    #[test]
    fn case_3() {
        let items = arr![[10, 1000]];
        let queries = vec![5];
        let expected = vec![0];
        assert_eq!(Solution::maximum_beauty(items, queries), expected);
    }
}
