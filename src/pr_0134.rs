use std::iter::zip;

pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut start, mut surplus, mut total_surplus) = (0, 0, 0);

        for (i, (v_gas, v_cost)) in zip(gas, cost).enumerate() {
            total_surplus += v_gas - v_cost;
            surplus += v_gas - v_cost;
            if surplus < 0 {
                surplus = 0;
                start = i + 1;
            }
        }
        if total_surplus < 0 {
            -1
        } else {
            i32::try_from(start).unwrap()
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
