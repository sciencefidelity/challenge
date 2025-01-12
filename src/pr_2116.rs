pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let (s, locked) = (s.as_bytes(), locked.as_bytes());
        let length = s.len();
        if length % 2 == 1 {
            return false;
        }
        let (mut open_brackets, mut unlocked) = (0, 0);
        for i in 0..length {
            if locked[i] == b'0' {
                unlocked += 1;
            } else if s[i] == b'(' {
                open_brackets += 1;
            } else if s[i] == b')' {
                if open_brackets > 0 {
                    open_brackets -= 1;
                } else if unlocked > 0 {
                    unlocked -= 1;
                } else {
                    return false;
                }
            }
        }
        let mut balance = 0;
        for i in (0..length).rev() {
            if locked[i] == b'0' {
                balance -= 1;
                unlocked -= 1;
            } else if s[i] == b'(' {
                balance += 1;
                open_brackets -= 1;
            } else if s[i] == b')' {
                balance -= 1;
            }
            if balance > 0 {
                return false;
            }
            if unlocked == 0 && open_brackets == 0 {
                break;
            }
        }
        if open_brackets > 0 {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_be_valid(
            "))()))".to_owned(),
            "010100".to_owned()
        ));
    }

    #[test]
    fn case_2() {
        assert!(Solution::can_be_valid("()()".to_owned(), "0000".to_owned()));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::can_be_valid(")".to_owned(), "0".to_owned()));
    }
}
