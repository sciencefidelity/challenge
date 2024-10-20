pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn parse_bool_expr(expression: String) -> bool {
        Self::parse(&mut expression.bytes())
    }

    fn parse(iter: &mut std::str::Bytes) -> bool {
        match iter.next() {
            Some(b't') => true,
            Some(b'f') => false,
            Some(b'!') => {
                iter.next();
                let result = !Self::parse(iter);
                iter.next();
                result
            }
            Some(op) => {
                let flag = op == b'&';
                let mut result = flag;
                while iter.next() != Some(b')') {
                    if Self::parse(iter) != flag {
                        result = !flag;
                    }
                }
                result
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(!Solution::parse_bool_expr("&(|(f))".to_owned()));
    }

    #[test]
    fn case_2() {
        assert!(Solution::parse_bool_expr("|(f,f,f,t)".to_owned()));
    }
}
