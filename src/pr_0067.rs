pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn add_binary(a: String, b: String) -> String {
        let n = a.len().max(b.len());
        let mut iter_a = a.chars();
        let mut iter_b = b.chars();
        let mut carry = '0';
        let mut sum_q = VecDeque::new();
        for _ in 0..n {
            match (
                iter_a.next_back().unwrap_or('0'),
                iter_b.next_back().unwrap_or('0'),
                carry,
            ) {
                ('0', '0', '0') => sum_q.push_front('0'),
                ('1', '0', '0') | ('0', '1', '0') | ('0', '0', '1') => {
                    sum_q.push_front('1');
                    carry = '0';
                }
                ('1', '1', '0') | ('1', '0', '1') | ('0', '1', '1') => {
                    sum_q.push_front('0');
                    carry = '1';
                }
                (_, _, _) => sum_q.push_front('1'),
            }
        }
        if carry == '1' {
            sum_q.push_front('1');
        }
        sum_q.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::add_binary("11".to_owned(), "1".to_owned()),
            "100".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
            "10101".to_owned()
        );
    }
}
