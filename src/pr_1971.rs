pub struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut v = vec![-1; usize::try_from(n).unwrap()];
        for edge in edges {
            let p1 = Self::find(&v, edge[0]);
            let p2 = Self::find(&v, edge[1]);
            if p1 != p2 {
                v[usize::try_from(p2).unwrap()] = p1;
            }
        }
        Self::find(&v, source) == Self::find(&v, destination)
    }

    fn find(v: &[i32], i: i32) -> i32 {
        let val = v[usize::try_from(i).unwrap()];
        if val < 0 {
            i
        } else {
            Self::find(v, val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert!(Solution::valid_path(3, arr![[0, 1], [1, 2], [2, 0]], 0, 2));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::valid_path(
            6,
            arr![[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]],
            0,
            5
        ));
    }
}
