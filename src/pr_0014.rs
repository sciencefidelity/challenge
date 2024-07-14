pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            String::new()
        } else {
            strs.into_iter()
                .reduce(|acc, cur| {
                    acc.chars()
                        .zip(cur.chars())
                        .take_while(|(a, c)| a == c)
                        .map(|(c, _)| c)
                        .collect()
                })
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_owned(),
                "flow".to_owned(),
                "flight".to_owned()
            ]),
            "fl".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_owned(),
                "racecar".to_owned(),
                "car".to_owned()
            ]),
            "".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "".to_owned());
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["".to_owned()]),
            "".to_owned()
        );
    }

    #[test]
    fn case_5() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["a".to_owned()]),
            "a".to_owned()
        );
    }
}
