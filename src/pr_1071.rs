pub struct Solution;

impl Solution {
    fn gcd(x: usize, y: usize) -> usize {
        println!("{x} {y}");
        match y {
            0 => return x,
            _ => return Self::gcd(y, x % y),
        }
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{str1}{str2}") != format!("{str2}{str1}") {
            return String::new();
        }

        let gcd_length = Self::gcd(str1.len(), str2.len());
        println!("{}", gcd_length);
        str1[..gcd_length].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_owned(), "ABC".to_owned()),
            "ABC".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_owned(), "ABAB".to_owned()),
            "AB".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_owned(), "CODE".to_owned()),
            "".to_owned()
        );
    }
}
