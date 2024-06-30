pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_surplus = 0;
        let mut surplus = 0;
        let mut start = 0;

        for i in 0..gas.len() {
            total_surplus += gas[i] - cost[i];
            surplus += gas[i] - cost[i];
            if surplus < 0 {
                surplus = 0;
                start = i + 1;
            }
        }
        if total_surplus < 0 {
            -1
        } else {
            start as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
