#![allow(clippy::cast_possible_truncation)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn compressed_string(word: String) -> String {
        word.into_bytes()
            .chunk_by(|a, b| a == b)
            .flat_map(|chunk| {
                chunk
                    .chunks(9)
                    .flat_map(|c| [Into::<char>::into(c.len() as u8 + b'0'), chunk[0].into()])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::compressed_string("abcde".to_owned()),
            "1a1b1c1d1e".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_owned()),
            "9a5a2b".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::compressed_string("aaaaaaaaay".to_owned()),
            "9a1y".to_owned()
        );
    }
}
