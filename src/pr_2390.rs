pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn remove_stars(s: String) -> String {
        s.chars()
            .fold(String::with_capacity(s.len()), |mut stack, c| {
                match c {
                    '*' => {
                        stack.pop();
                    }
                    c => stack.push(c),
                };
                stack
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            "lecoe".to_owned(),
            Solution::remove_stars("leet**cod*e".to_owned())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            String::new(),
            Solution::remove_stars("erase*****".to_owned())
        );
    }
}
