pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut counter = vec![0; usize::try_from(n + 1).unwrap()];
        for pair in trust {
            counter[usize::try_from(pair[0]).unwrap()] -= 1;
            counter[usize::try_from(pair[1]).unwrap()] += 1;
        }
        counter
            .into_iter()
            .position(|cnt| cnt == n - 1)
            .map_or(-1, |i| i.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(Solution::find_judge(2, arr![[1, 2]]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::find_judge(3, arr![[1, 3], [2, 3]]), 3);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::find_judge(3, arr![[1, 3], [2, 3], [3, 1]]), -1);
    }
}
