use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: HashSet<Vec<i32>> = HashSet::new();
        let (mut n, mut p, mut z) = (vec![], vec![], vec![]);
        for num in nums {
            // match num {
            //     1.. => p.push(num),
            //     ..0 => n.push(num),
            //     0 => z.push(num),
            // }
            if num > 0 {
                p.push(num);
            } else if num < 0 {
                n.push(num);
            } else {
                z.push(num);
            }
        }
        let (set_n, set_p): (HashSet<i32>, HashSet<i32>) =
            (HashSet::from_iter(n.clone()), HashSet::from_iter(p.clone()));
        if z.len() > 0 {
            for num in set_p.iter() {
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
                let target = -1 * (n[i] + n[j]);
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
        let output = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(output.len(), 2);
        assert!(output.contains(&vec![-1, -1, 2]));
        assert!(output.contains(&vec![-1, 0, 1]));
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
