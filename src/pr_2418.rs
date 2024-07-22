pub struct Solution;

// impl Solution {
//     pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
//         let mut people: Vec<(String, i32)> = names.into_iter().zip(heights).collect();
//         people.sort_unstable_by_key(|(_, height)| -height);
//         people.into_iter().map(|(name, _)| name).collect()
//     }
// }

// use std::collections::BTreeMap;
//
// impl Solution {
//     pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
//         let people: BTreeMap<_, _> = heights.into_iter().zip(names).collect();
//         people.into_iter().rev().map(|(_, name)| name).collect()
//     }
// }

use std::collections::BinaryHeap;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let n = names.len();
        let mut heap = BinaryHeap::from(heights.into_iter().zip(names).collect::<Vec<_>>());
        let mut names = Vec::with_capacity(n);
        while let Some(person) = heap.pop() {
            names.push(person.1);
        }
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let names = vec!["Mary".to_owned(), "John".to_owned(), "Emma".to_owned()];
        let heights = vec![180, 165, 170];
        let expected = vec!["Mary".to_owned(), "Emma".to_owned(), "John".to_owned()];
        assert_eq!(expected, Solution::sort_people(names, heights));
    }

    #[test]
    fn case_2() {
        let names = vec!["Alice".to_owned(), "Bob".to_owned(), "Bob".to_owned()];
        let heights = vec![155, 185, 150];
        let expected = vec!["Bob".to_owned(), "Alice".to_owned(), "Bob".to_owned()];
        assert_eq!(expected, Solution::sort_people(names, heights));
    }
}
