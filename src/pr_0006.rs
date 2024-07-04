pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut step = if num_rows == 1 { 0 } else { -1 };
        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        let mut i = 0i32;

        for c in s.chars() {
            rows[i as usize].push(c);
            if i == 0 || i == (num_rows as i32 - 1) {
                step = -step;
            }
            i += step;
        }
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
