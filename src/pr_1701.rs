pub struct Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let n = i32::try_from(customers.len()).unwrap();
        let mut next_idle_time = 0;
        let mut net_wait_time = 0;
        for customer in customers {
            let (arrival, time) = (i64::from(customer[0]), i64::from(customer[1]));
            next_idle_time = arrival.max(next_idle_time) + time;
            net_wait_time += i32::try_from(next_idle_time - arrival).unwrap();
        }
        f64::from(net_wait_time) / f64::from(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        assert_eq!(Solution::average_waiting_time(vec![vec![1, 2]]), 2.0);
    }

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]),
            5.0
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]),
            3.25
        );
    }
}
