use std::collections::HashMap;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let mut map = HashMap::with_capacity(arr.len());
        for s in &arr {
            map.entry(s).and_modify(|count| *count += 1).or_insert(1);
        }
        for s in &arr {
            if let Some(count) = map.get(&s) {
                if *count == 1 {
                    k -= 1;
                }
                if k == 0 {
                    return s.to_owned();
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::stringify;

    #[test]
    fn case_1() {
        assert_eq!(
            "a".to_owned(),
            Solution::kth_distinct(stringify(&["d", "b", "c", "b", "c", "a"]), 2)
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            "aaa".to_owned(),
            Solution::kth_distinct(stringify(&["aaa", "aa", "a"]), 1)
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            String::new(),
            Solution::kth_distinct(stringify(&["a", "b", "a"]), 3)
        );
    }
}
