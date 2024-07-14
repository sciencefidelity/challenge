use std::{cmp::Ordering, collections::HashSet};

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: HashSet<Vec<i32>> = HashSet::new();
        let (mut n, mut p, mut z) = (vec![], vec![], vec![]);
        for num in nums {
            match num.cmp(&0) {
                Ordering::Less => p.push(num),
                Ordering::Greater => n.push(num),
                Ordering::Equal => z.push(num),
            }
        }
        let (set_n, set_p): (HashSet<i32>, HashSet<i32>) =
            (HashSet::from_iter(n.clone()), HashSet::from_iter(p.clone()));
        if !z.is_empty() {
            for num in &set_p {
                if set_n.contains(&(-1 * num)) {
                    res.insert(vec![-1 * num, 0, *num]);
                }
            }
        }
        if z.len() >= 3 {
            res.insert(vec![0, 0, 0]);
        }
        for i in 0..n.len() {
            for j in (i + 1)..n.len() {
                let target = -(n[i] + n[j]);
                if set_p.contains(&target) {
                    let mut vec = vec![n[i], n[j], target];
                    vec.sort_unstable();
                    res.insert(vec);
                }
            }
        }
        for i in 0..p.len() {
            for j in (i + 1)..p.len() {
                let target = -1 * (p[i] + p[j]);
                if set_n.contains(&target) {
                    let mut vec = vec![p[i], p[j], target];
                    vec.sort_unstable();
                    res.insert(vec);
                }
            }
        }
        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut output = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        output.sort_unstable();
        output[0].sort_unstable();
        output[1].sort_unstable();
        println!("{output:?}");
        assert_eq!(output, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
