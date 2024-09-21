pub struct Solution;

impl Solution {
    // pub fn lexical_order(n: i32) -> Vec<i32> {
    //     let mut result = Vec::with_capacity(n as usize);
    //     for start in 1..=9 {
    //         Self::generate_lexical_numbers(start, n, &mut result)
    //     }
    //     result
    // }
    //
    // fn generate_lexical_numbers(current_number: i32, limit: i32, result: &mut Vec<i32>) {
    //     if current_number > limit {
    //         return;
    //     }
    //     result.push(current_number);
    //     for next_digit in 0..=9 {
    //         let next_number = current_number * 10 + next_digit;
    //         if next_number <= limit {
    //             Self::generate_lexical_numbers(next_number, limit, result);
    //         } else {
    //             break;
    //         }
    //     }
    // }

    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(usize::try_from(n).unwrap());
        let mut current_number = 1;
        for _ in 0..n {
            result.push(current_number);
            if current_number * 10 <= n {
                current_number *= 10;
            } else {
                while current_number % 10 == 9 || current_number >= n {
                    current_number /= 10;
                }
                current_number += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::lexical_order(13),
            [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::lexical_order(2), [1, 2]);
    }
}
