pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => match stack.pop() {
                    Some(opening_brace) if Self::is_match(opening_brace, c) => continue,
                    _ => return false,
                },
                _ => continue,
            }
        }
        matches!(stack.len(), 0)
    }

    const fn is_match(opening_brace: char, closing_brace: char) -> bool {
        matches!(
            (opening_brace, closing_brace),
            ('(', ')') | ('[', ']') | ('{', '}')
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn case_2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_valid("{]".to_string()));
    }
}
