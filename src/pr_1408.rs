pub struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let joined = words.join(" ");
        words
            .into_iter()
            .filter(|word| joined.matches(word).nth(1).is_some())
            .collect()
    }
}
