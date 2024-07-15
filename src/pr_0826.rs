pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let max_ability = worker.iter().max().unwrap();
        let mut jobs = vec![0; usize::try_from(*max_ability + 1).unwrap()];

        for (i, d) in difficulty.into_iter().enumerate() {
            if d <= *max_ability {
                jobs[usize::try_from(d).unwrap()] =
                    jobs[usize::try_from(d).unwrap()].max(profit[i]);
            }
        }

        for i in 1..=*max_ability {
            jobs[usize::try_from(i).unwrap()] =
                jobs[usize::try_from(i).unwrap()].max(jobs[usize::try_from(i - 1).unwrap()]);
        }

        let mut net_profit = 0;
        for ability in worker {
            net_profit += jobs[usize::try_from(ability).unwrap()];
        }
        net_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            ),
            100
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25]),
            0
        );
    }
}
