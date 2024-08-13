pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len());
        for token in tokens {
            if let Ok(num) = token.parse() {
                stack.push(num);
            } else {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(Self::eval_token(lhs, rhs, &token));
            }
        }
        stack[0]
    }

    fn eval_token(lhs: i32, rhs: i32, op: &str) -> i32 {
        match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            _ => panic!("invalid token"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tokens = ["2", "1", "+", "3", "*"]
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(9, Solution::eval_rpn(tokens));
    }

    #[test]
    fn case_2() {
        let tokens = ["4", "13", "5", "/", "+"]
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(6, Solution::eval_rpn(tokens));
    }

    #[test]
    fn case_3() {
        let tokens = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        assert_eq!(22, Solution::eval_rpn(tokens));
    }
}
