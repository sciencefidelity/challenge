pub struct Solution;

// use std::collections::{BinaryHeap, HashMap};
//
// impl Solution {
//     #[allow(clippy::needless_pass_by_value)]
//     pub fn minimum_pushes(word: String) -> i32 {
//         let mut freq = HashMap::with_capacity(26);
//         for b in word.bytes() {
//             freq.entry(b).and_modify(|count| *count += 1).or_insert(1);
//         }
//         let mut heap = BinaryHeap::with_capacity(freq.len());
//         for entry in freq {
//             heap.push(entry.1);
//         }
//         let mut count = 0;
//         let mut i = 0;
//         while let Some(freq) = heap.pop() {
//             count += (1 + i / 8) * freq;
//             i += 1;
//         }
//         count
//     }
// }

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn minimum_pushes(word: String) -> i32 {
        const A: usize = 97;
        let freq = &mut [0; 26];
        for b in word.bytes() {
            freq[usize::from(b) - A] += 1;
        }
        freq.sort_unstable_by(|a, b| b.cmp(a));
        let mut count = 0;
        let mut i = 0;
        while i < freq.len() {
            if freq[i] == 0 {
                break;
            }
            count += (i / 8 + 1) * freq[i];
            i += 1;
        }
        i32::try_from(count).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_owned()));
    }

    #[test]
    fn case_2() {
        assert_eq!(12, Solution::minimum_pushes("xyzxyzxyzxyz".to_owned()));
    }

    #[test]
    fn case_3() {
        assert_eq!(
            24,
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_owned())
        );
    }
}
