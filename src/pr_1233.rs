use std::collections::HashSet;

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let n = folder.len();
        let set = HashSet::<String>::from_iter(folder.clone());
        let mut result = Vec::with_capacity(n);
        for f in &folder {
            let mut prefix = f.as_str();
            let mut is_subfolder = false;
            while !prefix.is_empty() {
                if let Some(pos) = prefix.rfind('/') {
                    prefix = &prefix[..pos];
                    if set.contains(prefix) {
                        is_subfolder = true;
                        break;
                    }
                } else {
                    break;
                }
            }
            if !is_subfolder {
                result.push(f.to_string());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::remove_subfolders(arr!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]),
            arr!["/a", "/c/d", "/c/f"]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::remove_subfolders(arr!["/a", "/a/b/c", "/a/b/d"]),
            arr!["/a"]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::remove_subfolders(arr!["/a/b/c", "/a/b/ca", "/a/b/d"]),
            arr!["/a/b/c", "/a/b/ca", "/a/b/d"]
        );
    }
}
