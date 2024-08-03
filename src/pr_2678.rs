pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .filter(|p| &p[11..13] > "60")
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            2,
            Solution::count_seniors(vec![
                "7868190130M7522".to_owned(),
                "5303914400F9211".to_owned(),
                "9273338290F4010".to_owned()
            ])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            0,
            Solution::count_seniors(vec![
                "1313579440F2036".to_owned(),
                "2921522980M5644".to_owned(),
            ])
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            0,
            Solution::count_seniors(vec![
                "1313579440F6036".to_owned(),
                "2921522980M5644".to_owned(),
            ])
        );
    }
}
