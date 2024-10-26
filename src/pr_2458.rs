#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
use b_tree::TreeNode;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Solution;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn tree_queries(root: OptNode, queries: Vec<i32>) -> Vec<i32> {
        let (mut levels, mut map) = (Vec::new(), HashMap::new());
        Self::map(root, &mut levels, &mut map, 0);
        queries
            .into_iter()
            .map(|q| {
                let &level = map.get(&q).unwrap();
                if levels[level].len() == 1 {
                    (level - 1) as i32
                } else {
                    levels[level]
                        .iter()
                        .filter(|&(val, _)| (*val != q))
                        .map(|(_, l)| *l)
                        .max()
                        .unwrap() as i32
                }
            })
            .collect()
    }

    fn map(
        node: OptNode,
        levels: &mut Vec<Vec<(i32, usize)>>,
        map: &mut HashMap<i32, usize>,
        level: usize,
    ) -> usize {
        if let Some(node) = node {
            if levels.len() == level {
                levels.push(Vec::new());
            }
            let max_level = Self::map(node.borrow().left.clone(), levels, map, level + 1).max(
                Self::map(node.borrow().right.clone(), levels, map, level + 1),
            );
            levels[level].push((node.borrow().val, max_level));
            map.insert(node.borrow().val, level);
            max_level
        } else {
            level - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(3),
            Some(4),
            Some(2),
            None,
            Some(6),
            Some(5),
            None,
            None,
            None,
            None,
            None,
            Some(7),
        ]);
        assert_eq!(Solution::tree_queries(root, vec![4]), vec![2]);
    }

    #[test]
    fn case_2() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(8),
            Some(9),
            Some(2),
            Some(1),
            Some(3),
            Some(7),
            Some(4),
            Some(6),
        ]);
        assert_eq!(
            Solution::tree_queries(root, vec![3, 2, 4, 8]),
            vec![3, 2, 3, 2]
        );
    }
}
