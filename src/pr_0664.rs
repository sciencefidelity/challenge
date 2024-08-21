use std::cmp::min;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn strange_printer(s: String) -> i32 {
        let bytes = Self::remove_duplicates(s.as_bytes());
        let n = bytes.len();
        let mut min_turns = vec![vec![0; n]; n];
        for (i, b) in min_turns.iter_mut().enumerate() {
            b[i] = 1;
        }
        for length in 2..=n {
            let mut start = 0;
            while start + length - 1 < n {
                let end = start + length - 1;
                min_turns[start][end] = length;
                for split in 0..length - 1 {
                    let mut total_turns =
                        min_turns[start][start + split] + min_turns[start + split + 1][end];
                    if bytes[start + split] == bytes[end] {
                        total_turns -= 1;
                    }
                    min_turns[start][end] = min(min_turns[start][end], total_turns);
                }
                start += 1;
            }
        }
        i32::try_from(min_turns[0][n - 1]).unwrap()
    }

    fn remove_duplicates(bytes: &[u8]) -> Vec<u8> {
        let mut unique_bytes = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let current_byte = bytes[i];
            unique_bytes.push(current_byte);
            while i < bytes.len() && bytes[i] == current_byte {
                i += 1;
            }
        }
        unique_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::strange_printer("aaabbb".to_string()), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::strange_printer("aba".to_string()), 2);
    }
}
