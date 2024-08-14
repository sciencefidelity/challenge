pub struct Solution;

const BYTE_DIGIT_OFFSET: u8 = 50;
static KEYPAD: [&[char]; 8] = [
    &['a', 'b', 'c'],
    &['d', 'e', 'f'],
    &['g', 'h', 'i'],
    &['j', 'k', 'l'],
    &['m', 'n', 'o'],
    &['p', 'q', 'r', 's'],
    &['t', 'u', 'v'],
    &['w', 'x', 'y', 'z'],
];

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut output = Vec::new();
        if digits.is_empty() {
            return output;
        }
        Self::dfs(&mut output, digits.as_bytes(), &mut Vec::new());
        output
    }

    fn dfs(output: &mut Vec<String>, digits: &[u8], comb: &mut Vec<char>) {
        if digits.is_empty() {
            output.push(comb.iter().collect());
            return;
        }
        for c in KEYPAD[usize::from(digits[0] - BYTE_DIGIT_OFFSET)] {
            comb.push(*c);
            Self::dfs(output, &digits[1..], comb);
            comb.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::stringify;

    #[test]
    fn case_1() {
        assert_eq!(
            stringify(&["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]),
            Solution::letter_combinations("23".to_owned())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(stringify(&[]), Solution::letter_combinations(String::new()));
    }

    #[test]
    fn case_3() {
        assert_eq!(
            stringify(&["a", "b", "c"]),
            Solution::letter_combinations("2".to_owned())
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            stringify(&[
                "adp", "adq", "adr", "ads", "aep", "aeq", "aer", "aes", "afp", "afq", "afr", "afs",
                "bdp", "bdq", "bdr", "bds", "bep", "beq", "ber", "bes", "bfp", "bfq", "bfr", "bfs",
                "cdp", "cdq", "cdr", "cds", "cep", "ceq", "cer", "ces", "cfp", "cfq", "cfr", "cfs"
            ]),
            Solution::letter_combinations("237".to_owned())
        );
    }
}
