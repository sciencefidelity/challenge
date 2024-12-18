use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut freq = HashMap::with_capacity(s.len());
        let mut max_heap = BinaryHeap::with_capacity(s.len());
        for ch in s.chars() {
            freq.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        }
        for ch in freq.keys() {
            max_heap.push(*ch);
        }
        let mut result = String::new();
        while !max_heap.is_empty() {
            let ch = max_heap.pop().unwrap();
            let &count = freq.get(&ch).unwrap();
            let used = count.min(repeat_limit);
            for _ in 0..used {
                result.push(ch);
            }
            freq.entry(ch).and_modify(|count| *count -= used);
            if *freq.get(&ch).unwrap() > 0 && !max_heap.is_empty() {
                let next_char = max_heap.pop().unwrap();
                result.push(next_char);
                freq.entry(next_char).and_modify(|count| *count -= 1);
                if *freq.get(&next_char).unwrap() > 0 {
                    max_heap.push(next_char);
                }
                max_heap.push(ch);
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
            Solution::repeat_limited_string("cczazcc".to_owned(), 3),
            "zzcccac".to_owned()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::repeat_limited_string("aababab".to_owned(), 2),
            "bbabaa".to_owned()
        );
    }
}
