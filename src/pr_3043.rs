pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr1.sort_unstable();
        arr2.sort_unstable();
        arr1.dedup();
        arr2.dedup();
        let trie = arr1.into_iter().fold(Trie::new(), |mut trie, v| {
            trie.insert(usize::try_from(v).unwrap());
            trie
        });
        let mut max = 0;
        for v in arr2.into_iter().rev() {
            max = if let Some(m) = trie.prefix_len(usize::try_from(v).unwrap(), max) {
                m.max(max)
            } else {
                return i32::try_from(max).unwrap();
            }
        }
        i32::try_from(max).unwrap()
    }
}

#[derive(Clone, Default)]
struct Trie([Option<Box<Trie>>; 10]);

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, num: usize) {
        let digits = usize::try_from(num.ilog10()).unwrap();
        let mut current = self;
        for col in (0..=digits).rev() {
            let pow = 10_usize.pow(u32::try_from(col).unwrap());
            let digit = (num % (pow * 10)) / pow;
            if current.0[digit].is_none() {
                current.0[digit] = Some(Box::new(Self::new()));
            }
            current = current.0[digit].as_mut().unwrap();
        }
    }

    pub fn prefix_len(&self, num: usize, min_threshold: usize) -> Option<usize> {
        let digits = usize::try_from(num.ilog10()).unwrap();
        if min_threshold > digits {
            return None;
        }
        let mut current = self;
        for col in (0..=digits).rev() {
            let pow = 10_usize.pow(u32::try_from(col).unwrap());
            let digit = (num % (pow * 10)) / pow;
            if current.0[digit].is_none() {
                return Some(digits - col);
            }
            current = current.0[digit].as_ref().unwrap();
        }
        Some(digits + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
            3
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
            0
        );
    }
}
