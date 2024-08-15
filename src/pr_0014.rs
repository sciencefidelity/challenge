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
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::longest_common_prefix(arr!["flower", "flow", "flight"]),
            "fl".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_common_prefix(arr!["dog", "racecar", "car"]),
            String::new()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::longest_common_prefix(vec![]), String::new());
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::longest_common_prefix(arr![""]), String::new());
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::longest_common_prefix(arr!["a"]), "a".to_owned());
    }
}
