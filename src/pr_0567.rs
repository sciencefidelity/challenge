pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (freqs1, freqs2) = (&mut [0; 26], &mut [0; 26]);
        for b in s1.bytes() {
            freqs1[usize::from(b - b'a')] += 1;
        }
        let s2 = s2.as_bytes();
        for (i, b) in s2.iter().enumerate() {
            freqs2[usize::from(b - b'a')] += 1;
            if i >= s1.len() {
                freqs2[usize::from(s2[i - s1.len()] - b'a')] -= 1;
            }
            if freqs1 == freqs2 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(Solution::check_inclusion(
            "ab".to_owned(),
            "eidbaooo".to_owned()
        ));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::check_inclusion(
            "ab".to_owned(),
            "eidboaoo".to_owned()
        ));
    }
}
