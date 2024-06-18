pub struct Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let max_ability = worker.iter().max().unwrap();
        let mut jobs = vec![0; *max_ability as usize + 1];

        for i in 0..difficulty.len() {
            if difficulty[i] <= *max_ability {
                jobs[difficulty[i] as usize] = jobs[difficulty[i] as usize].max(profit[i]);
            }
        }

        for i in 1..=*max_ability {
            jobs[i as usize] = jobs[i as usize].max(jobs[(i - 1) as usize]);
        }

        let mut net_profit = 0;
        for ability in worker {
            net_profit += jobs[ability as usize];
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
