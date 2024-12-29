pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let word_length = words[0].len();
        let target_length = target.len();
        let mut char_frequency = vec![vec![0; 26]; word_length];
        for word in words {
            for (j, freq) in char_frequency.iter_mut().enumerate().take(word_length) {
                freq[usize::from(word.as_bytes()[j] - b'a')] += 1;
            }
        }
        let mut prev_count = vec![0_i64; target_length + 1];
        let mut curr_count = vec![0_i64; target_length + 1];
        prev_count[0] = 1;
        for curr_word in 1..=word_length {
            curr_count.clone_from(&prev_count);
            for curr_target in 1..=target_length {
                let curr_pos = usize::from(target.as_bytes()[curr_target - 1] - b'a');
                curr_count[curr_target] +=
                    (char_frequency[curr_word - 1][curr_pos] * prev_count[curr_target - 1]) % MOD;
                curr_count[curr_target] %= MOD;
            }
            prev_count.clone_from(&curr_count);
        }
        i32::try_from(curr_count[target_length]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::num_ways(arr!["acca", "bbbb", "caca"], "aba".to_owned()),
            6
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::num_ways(arr!["abba", "baab"], "bab".to_owned()),
            4
        );
    }
}
