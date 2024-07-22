pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for dir in path.split('/') {
            match dir {
                "" | "." => continue,
                ".." => {
                    stack.pop();
                }
                _ => stack.push(dir),
            }
        }
        String::from('/') + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            "/home".to_owned(),
            Solution::simplify_path("/home/".to_owned())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            "/home/foo".to_owned(),
            Solution::simplify_path("/home//foo/".to_owned())
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            "/home/user/Pictures".to_owned(),
            Solution::simplify_path("/home/user/Documents/../Pictures".to_owned())
        );
    }

    #[test]
    fn case_4() {
        assert_eq!("/".to_owned(), Solution::simplify_path("/../".to_owned()));
    }

    #[test]
    fn case_5() {
        assert_eq!(
            "/.../b/d".to_owned(),
            Solution::simplify_path("/.../a/../b/c/../d/./".to_owned())
        );
    }

    fn case_6() {
        assert_eq!(
            "/home/.".to_owned(),
            Solution::simplify_path("/home/".to_owned())
        );
    }

    fn case_7() {
        assert_eq!(
            "/home/..".to_owned(),
            Solution::simplify_path("/".to_owned())
        );
    }
}
