pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_the_longest_substring(s: String) -> i32 {
        let (mut prefix, mut mp, mut result) = (0_usize, vec![-1; 32], 0);
        for (i, b) in s.bytes().enumerate() {
            let i = i32::try_from(i).unwrap();
            prefix ^= match b {
                b'a' => 1 << 0,
                b'e' => 1 << 1,
                b'i' => 1 << 2,
                b'o' => 1 << 3,
                b'u' => 1 << 4,
                _ => 0,
            };
            if mp[prefix] == -1 && prefix != 0 {
                mp[prefix] = i;
            }
            result = result.max(i - mp[prefix]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_the_longest_substring("eleetminicoworoep".to_owned()),
            13
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::find_the_longest_substring("leetcodeisgreat".to_owned()),
            5
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::find_the_longest_substring("bcbcbc".to_owned()), 6);
    }
}
