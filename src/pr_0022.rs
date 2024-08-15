pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let (n, mut output) = (usize::try_from(n).unwrap(), Vec::new());
        Self::backtrack(&mut output, 0, 0, &mut String::new(), n);
        output
    }

    pub fn backtrack(
        output: &mut Vec<String>,
        left: usize,
        right: usize,
        comb: &mut String,
        n: usize,
    ) {
        if comb.len() == n * 2 {
            output.push(comb.clone());
            return;
        }
        if left < n {
            comb.push('(');
            Self::backtrack(output, left + 1, right, comb, n);
            comb.pop();
        }
        if right < left {
            comb.push(')');
            Self::backtrack(output, left, right + 1, comb, n);
            comb.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            arr!["((()))", "(()())", "(())()", "()(())", "()()()"],
            Solution::generate_parenthesis(3)
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(arr!["()"], Solution::generate_parenthesis(1));
    }
}
