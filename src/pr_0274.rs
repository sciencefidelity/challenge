pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut count = vec![0; n + 1];

        for citation in citations.iter() {
            if *citation > n as i32 {
                count[n] += 1;
            } else {
                count[*citation as usize] += 1;
            }
        }

        if count[n] >= n {
            return n as i32;
        }

        for i in (0..n).rev() {
            count[i] += count[i + 1];
            if count[i] >= i {
                return i as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }
}
