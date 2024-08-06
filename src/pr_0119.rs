pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = usize::try_from(row_index).unwrap();
        let mut output = vec![];
        output.push(1);

        for i in 1..=row_index {
            let tmp = usize::try_from(output[i - 1]).unwrap() * (row_index + 1 - i) / i;
            output.push(i32::try_from(tmp).unwrap());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let expected = vec![1, 3, 3, 1];
        assert_eq!(expected, Solution::get_row(3));
    }

    #[test]
    fn case_2() {
        let expected = vec![1];
        assert_eq!(expected, Solution::get_row(0));
    }

    #[test]
    fn case_3() {
        let expected = vec![1, 1];
        assert_eq!(expected, Solution::get_row(1));
    }

    #[test]
    fn case_4() {
        let expected = vec![
            1, 30, 435, 4060, 27405, 142506, 593775, 2035800, 5852925, 14307150, 30045015,
            54627300, 86493225, 119759850, 145422675, 155117520, 145422675, 119759850, 86493225,
            54627300, 30045015, 14307150, 5852925, 2035800, 593775, 142506, 27405, 4060, 435, 30,
            1,
        ];
        assert_eq!(expected, Solution::get_row(30));
    }
}
