pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let (mut seen, mut stack, mut n_seen) = (vec![false; rooms.len()], Vec::from([0]), 1);
        seen[0] = true;
        while let Some(node) = stack.pop() {
            for key in &rooms[node] {
                let key = usize::try_from(*key).unwrap();
                if !seen[key] {
                    n_seen += 1;
                    if n_seen == rooms.len() {
                        return true;
                    }
                    seen[key] = true;
                    stack.push(key);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        assert!(Solution::can_visit_all_rooms(arr![[1], [2], [3], []]));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::can_visit_all_rooms(arr![
            [1, 3],
            [3, 0, 1],
            [2],
            [0]
        ]));
    }
}
