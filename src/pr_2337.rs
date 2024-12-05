pub struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        #[derive(Debug, Clone, Copy)]
        enum Dir {
            Left,
            Right,
        }

        fn tokens(s: String) -> impl Iterator<Item = (usize, Dir)> {
            s.into_bytes()
                .into_iter()
                .enumerate()
                .filter_map(|(i, byte)| match byte {
                    b'L' => Some((i, Dir::Left)),
                    b'R' => Some((i, Dir::Right)),
                    _ => None,
                })
        }

        let mut start_tokens = tokens(start);
        let mut target_tokens = tokens(target);
        loop {
            match (start_tokens.next(), target_tokens.next()) {
                (Some((i, Dir::Left)), Some((j, Dir::Left))) if i >= j => {}
                (Some((i, Dir::Right)), Some((j, Dir::Right))) if i <= j => {}
                (None, None) => break true,
                _ => break false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::can_change(
            "_L__R__R_".to_owned(),
            "L______RR".to_owned()
        ));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::can_change("R_L_".to_owned(), "__LR".to_owned()));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::can_change("_R".to_owned(), "R_".to_owned()));
    }
}
