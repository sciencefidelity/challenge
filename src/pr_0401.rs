pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = u32::try_from(turned_on).unwrap();
        let mut output = Vec::new();
        for i in 0_i32..12 {
            for j in 0_i32..60 {
                if i.count_ones() + j.count_ones() == turned_on {
                    output.push(format!("{i}:{j:02}"));
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stringify(strs: &[&str]) -> Vec<String> {
        strs.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn case_1() {
        let output = stringify(&[
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ]);
        assert_eq!(output, Solution::read_binary_watch(1));
    }

    #[test]
    fn case_2() {
        let output = stringify(&[]);
        assert_eq!(output, Solution::read_binary_watch(9));
    }
}
