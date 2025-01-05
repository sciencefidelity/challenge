#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let (n, s) = (s.len(), s.as_bytes());
        let mut dif_array = vec![0; n];
        for shift in shifts {
            if shift[2] == 1 {
                dif_array[shift[0] as usize] += 1;
                if shift[1] as usize + 1 < n {
                    dif_array[shift[1] as usize + 1] -= 1;
                }
            } else {
                dif_array[shift[0] as usize] -= 1;
                if shift[1] as usize + 1 < n {
                    dif_array[(shift[1] + 1) as usize] += 1;
                }
            }
        }
        let mut result = vec![b' '; n];
        let mut number_of_shifts: i32 = 0;
        for i in 0..n {
            number_of_shifts = (number_of_shifts + dif_array[i]) % 26;
            if number_of_shifts < 0 {
                number_of_shifts += 26;
            }
            result[i] = b'a' + (s[i] - b'a' + number_of_shifts as u8) % 26;
        }
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::shifting_letters("abc".to_owned(), arr![[0, 1, 0], [1, 2, 1], [0, 2, 1]]),
            "ace".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::shifting_letters("dztz".to_owned(), arr![[0, 0, 0], [1, 1, 1]]),
            "catz".to_owned()
        );
    }
}
