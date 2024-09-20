pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut results = Vec::new();
        Self::recurse(&expression, &mut results);
        results
    }

    pub fn recurse(expression: &str, results: &mut Vec<i32>) {
        if expression.is_empty() {
            return;
        }
        if expression.len() == 1
            || expression.len() == 2 && expression.as_bytes()[0].is_ascii_digit()
        {
            results.push(expression.parse().unwrap());
            return;
        }
        for (i, b) in expression.bytes().enumerate() {
            if b.is_ascii_digit() {
                continue;
            }
            let left_results = Self::diff_ways_to_compute(expression[..i].to_owned());
            let right_results = Self::diff_ways_to_compute(expression[i + 1..].to_owned());
            for left_value in &left_results {
                for right_value in &right_results {
                    let computed_result = match b {
                        b'+' => left_value + right_value,
                        b'-' => left_value - right_value,
                        b'*' => left_value * right_value,
                        _ => unreachable!(),
                    };
                    results.push(computed_result);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::diff_ways_to_compute("2-1-1".to_owned()),
            vec![2, 0]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::diff_ways_to_compute("2*3-4*5".to_owned()),
            vec![-34, -10, -14, -10, 10]
        );
    }
}
