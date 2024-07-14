pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut step = if num_rows == 1 { 0 } else { -1 };
        let mut rows: Vec<String> = vec![String::new(); usize::try_from(num_rows).unwrap()];
        let mut i = 0_i32;

        s.chars().for_each(|c| {
            rows[usize::try_from(i).unwrap()].push(c);
            if i == 0 || i == num_rows - 1 {
                step = -step;
            }
            i += step;
        });
        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_owned(), 3),
            "PAHNAPLSIIGYIR".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_owned(), 4),
            "PINALSIGYAHRPI".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::convert("A".to_owned(), 1), "A".to_owned());
    }
}
