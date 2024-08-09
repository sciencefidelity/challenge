use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let mut r_queue = VecDeque::with_capacity(n);
        let mut d_queue = VecDeque::with_capacity(n);
        for (i, b) in senate.bytes().enumerate() {
            if b == b'R' {
                r_queue.push_back(i);
            }
            if b == b'D' {
                d_queue.push_back(i);
            }
        }

        while !r_queue.is_empty() && !d_queue.is_empty() {
            let (r_idx, d_idx) = (r_queue.pop_front().unwrap(), d_queue.pop_front().unwrap());
            if r_idx < d_idx {
                r_queue.push_back(r_idx + n);
            } else {
                d_queue.push_back(d_idx + n);
            }
        }
        if r_queue.is_empty() {
            "Dire".to_owned()
        } else {
            "Radiant".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            "Radiant".to_owned(),
            Solution::predict_party_victory("RD".to_owned())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            "Dire".to_owned(),
            Solution::predict_party_victory("RDD".to_owned())
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            "Radiant".to_owned(),
            Solution::predict_party_victory("RRDRD".to_owned())
        );
    }
}
