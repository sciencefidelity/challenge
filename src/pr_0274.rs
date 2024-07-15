pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut count = vec![0; n + 1];

        for citation in citations {
            if citation > i32::try_from(n).unwrap() {
                count[n] += 1;
            } else {
                count[usize::try_from(citation).unwrap()] += 1;
            }
        }

        if count[n] >= n {
            return i32::try_from(n).unwrap();
        }

        for i in (0..n).rev() {
            count[i] += count[i + 1];
            if count[i] >= i {
                return i32::try_from(i).unwrap();
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
