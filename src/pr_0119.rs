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
            1,
            30,
            435,
            4060,
            27_405,
            142_506,
            593_775,
            2_035_800,
            5_852_925,
            14_307_150,
            30_045_015,
            54_627_300,
            86_493_225,
            119_759_850,
            145_422_675,
            155_117_520,
            145_422_675,
            119_759_850,
            86_493_225,
            54_627_300,
            30_045_015,
            14_307_150,
            5_852_925,
            2_035_800,
            593_775,
            142_506,
            27_405,
            4060,
            435,
            30,
            1,
        ];
        assert_eq!(expected, Solution::get_row(30));
    }
}
