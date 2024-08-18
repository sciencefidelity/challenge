pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = usize::try_from(n).unwrap();
        let mut uns = vec![0; n];
        uns[0] = 1;
        let mut idx = (0, 0, 0);
        let mul = &mut [2, 3, 5];
        for i in 1..n {
            uns[i] = *mul.iter().min().unwrap();
            if uns[i] == mul[0] {
                idx.0 += 1;
                mul[0] = uns[idx.0] * 2;
            }
            if uns[i] == mul[1] {
                idx.1 += 1;
                mul[1] = uns[idx.1] * 3;
            }
            if uns[i] == mul[2] {
                idx.2 += 1;
                mul[2] = uns[idx.2] * 5;
            }
        }
        uns[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(1, Solution::nth_ugly_number(1));
    }

    #[test]
    fn case_3() {
        assert_eq!(2_460_375, Solution::nth_ugly_number(600));
    }

    #[test]
    fn case_4() {
        assert_eq!(2_123_366_400, Solution::nth_ugly_number(1690));
    }
}
