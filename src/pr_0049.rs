use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let (mut i, n) = (0, &strs.len());
        let mut map: HashMap<[u8; 26], usize> = HashMap::with_capacity(*n);
        let mut solution: Vec<Vec<String>> = Vec::with_capacity(*n);

        for s in strs {
            let mut slice = [0; 26];
            for b in s.bytes() {
                slice[usize::from(b - b'a')] += 1;
            }
            if let Some(j) = map.get(&slice) {
                solution[*j].push(s.clone());
            } else {
                solution.push(Vec::new());
                solution[i].push(s.clone());
                i += 1;
                map.insert(slice, i - 1);
            }
            // map.entry(slice)
            //     .and_modify(|j| solution[*j].push(s.clone()))
            //     .or_insert_with(|| {
            //         solution.push(Vec::with_capacity(*n));
            //         solution[i].push(s.clone());
            //         i += 1;
            //         i - 1
            //     });
        }
        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let words = vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned(),
        ];
        let mut solution = Solution::group_anagrams(words);
        let expected = vec![
            vec!["bat".to_owned()],
            vec!["nat".to_owned(), "tan".to_owned()],
            vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()],
        ];
        solution.sort_by_key(Vec::len);
        solution
            .iter_mut()
            .for_each(|word_list| word_list.sort_unstable());
        assert_eq!(expected, solution);
    }

    #[test]
    fn case_2() {
        let words = vec![String::new()];
        let mut solution = Solution::group_anagrams(words);
        let expected = vec![vec![String::new()]];
        solution.sort_by_key(Vec::len);
        solution
            .iter_mut()
            .for_each(|word_list| word_list.sort_unstable());
        assert_eq!(expected, solution);
    }

    #[test]
    fn case_3() {
        let words = vec!["a".to_owned()];
        let mut solution = Solution::group_anagrams(words);
        let expected = vec![vec!["a".to_owned()]];
        solution.sort_by_key(Vec::len);
        solution
            .iter_mut()
            .for_each(|word_list| word_list.sort_unstable());
        assert_eq!(expected, solution);
    }
}
