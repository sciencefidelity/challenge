pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn min_remove_to_make_valid(s: String) -> String {
        let (mut left, mut right) = (Vec::new(), Vec::new());
        let mut result = String::new();
        for (i, b) in s.chars().enumerate() {
            match b {
                '(' => left.push(i),
                ')' => {
                    if left.is_empty() {
                        right.push(i);
                    } else {
                        left.pop();
                    }
                }
                _ => {}
            }
        }
        let (mut i, mut j) = (0, 0);
        for (k, c) in s.chars().enumerate() {
            if i < left.len() && left[i] == k {
                i += 1;
            } else if j < right.len() && right[j] == k {
                j += 1;
            } else {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_owned()),
            "lee(t(c)o)de".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::min_remove_to_make_valid("a)b(c)d".to_owned()),
            "ab(c)d".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::min_remove_to_make_valid("))((".to_owned()),
            String::new()
        );
    }
}
