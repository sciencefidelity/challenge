pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        trie.scores(&words)
    }
}

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    count: i32,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for b in word.bytes() {
            let idx = usize::from(b - b'a');
            if node.next[idx].is_none() {
                node.next[idx] = Some(Box::new(Self::new()));
            }
            node.next[idx].as_mut().unwrap().count += 1;
            node = node.next[idx].as_mut().unwrap();
        }
    }

    pub fn count(&mut self, s: &str) -> i32 {
        let (mut node, mut result) = (self, 0);
        for b in s.bytes() {
            let idx = usize::from(b - b'a');
            result += node.next[idx].as_ref().unwrap().count;
            node = node.next[idx].as_mut().unwrap();
        }
        result
    }

    pub fn scores(&mut self, words: &[String]) -> Vec<i32> {
        for word in words {
            self.insert(word);
        }
        let mut scores = vec![0; words.len()];
        for (i, score) in scores.iter_mut().enumerate() {
            *score = self.count(&words[i]);
        }
        scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::sum_prefix_scores(arr!["abc", "ab", "bc", "b"]),
            vec![5, 4, 3, 2]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::sum_prefix_scores(arr!["abcd"]), vec![4]);
    }
}
