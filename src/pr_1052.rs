pub struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let minutes = minutes.try_into().expect("unable to convert i32 to usize");
        let mut unrealized_customers = 0;

        for i in 0..minutes {
            unrealized_customers += customers[i] * grumpy[i];
        }
        let mut max_unrealized_customers = unrealized_customers;

        for i in minutes..customers.len() {
            unrealized_customers += customers[i] * grumpy[i];
            unrealized_customers -= customers[i - minutes] * grumpy[i - minutes];
            max_unrealized_customers = max_unrealized_customers.max(unrealized_customers);
        }
        let mut total_customers = max_unrealized_customers;

        for i in 0..customers.len() {
            total_customers += customers[i] * (1 - grumpy[i]);
        }
        total_customers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_satisfied(vec![1], vec![2], 1), 1);
    }
}
