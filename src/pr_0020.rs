pub struct Solution;

impl Solution {
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
        match stack.len() {
            0 => true,
            _ => false,
        }
    }

    fn is_match(opening_brace: char, closing_brace: char) -> bool {
        match (opening_brace, closing_brace) {
            ('(', ')') | ('[', ']') | ('{', '}') => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
    }

    #[test]
    fn case_2() {
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn case_3() {
        assert_eq!(false, Solution::is_valid("{]".to_string()));
    }
}
