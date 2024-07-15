pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (n, mut i, mut j) = (chars.len(), 0, 0);
        let mut c;
        let mut left;
        while i < n {
            c = chars[i];
            left = i;
            while i < n && chars[i] == c {
                i += 1;
            }
            chars[j] = c;
            j += 1;
            if (i - left) > 1 {
                let nums = (i - left).to_string().into_bytes();
                for num in nums {
                    chars[j] = num as char;
                    j += 1;
                }
            }
        }
        chars.truncate(j);
        i32::try_from(chars.len()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut chars), 6);
        assert_eq!(chars, vec!['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn case_2() {
        let mut chars = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);
        assert_eq!(chars, vec!['a']);
    }

    #[test]
    fn case_3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(Solution::compress(&mut chars), 4);
        assert_eq!(chars, vec!['a', 'b', '1', '2']);
    }
}
