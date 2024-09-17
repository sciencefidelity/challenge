use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = HashMap::with_capacity(s1.len() + s2.len());
        for word in s1.split_whitespace().chain(s2.split_whitespace()) {
            map.entry(word).and_modify(|count| *count += 1).or_insert(1);
        }
        map.into_iter()
            .filter(|(_, count)| *count == 1)
            .map(|(key, _)| key.to_owned())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::uncommon_from_sentences(
            "this apple is sweet".to_owned(),
            "this apple is sour".to_owned(),
        );
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"sweet".to_owned()));
        assert!(result.contains(&"sour".to_owned()));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::uncommon_from_sentences("apple apple".to_owned(), "banana".to_owned()),
            vec!["banana"]
        );
    }
}
