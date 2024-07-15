use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::climber(n, &mut memo)
    }

    #[allow(clippy::option_if_let_else)]
    fn climber(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n - 1 <= 1 {
            return n;
        }
        if let Some(value) = memo.get(&n) {
            *value
        } else {
            let value = Self::climber(n - 1, memo) + Self::climber(n - 2, memo);
            memo.insert(n, value);
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
