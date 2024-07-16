use b_tree::TreeNode;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;

// impl Solution {
//     pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Tree {
//         let mut map = HashMap::new();
//         let mut set = HashSet::new();
//         let mut root_num = -1;
//         for node in &descriptions {
//             map.insert(node[0], (-1, -1));
//             set.insert(node[1]);
//         }
//         for node in &descriptions {
//             map.entry(node[0]).and_modify(|(left, right)| {
//                 if node[2] == 1 {
//                     *left = node[1];
//                 } else {
//                     *right = node[1];
//                 }
//             });
//             if !set.contains(&node[0]) {
//                 root_num = node[0];
//             }
//         }
//
//         let mut queue = VecDeque::new();
//         let root_node = Rc::new(RefCell::new(TreeNode::new(root_num)));
//         queue.push_back(root_node.clone());
//         while let Some(curr_node) = queue.pop_front() {
//             let (left_child_num, right_child_num) =
//                 map.get(&curr_node.clone().borrow().val).unwrap();
//             if *left_child_num > -1 {
//                 let left_node = Rc::new(RefCell::new(TreeNode::new(*left_child_num)));
//                 if map.contains_key(left_child_num) {
//                     queue.push_back(left_node.clone());
//                 }
//                 curr_node.clone().borrow_mut().left = Some(left_node);
//             }
//             if *right_child_num > -1 {
//                 let right_node = Rc::new(RefCell::new(TreeNode::new(*right_child_num)));
//                 if map.contains_key(right_child_num) {
//                     queue.push_back(right_node.clone());
//                 }
//                 curr_node.clone().borrow_mut().right = Some(right_node);
//             }
//         }
//         println!("{root_node:#?}");
//         Some(root_node)
//     }
// }

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Tree {
        let mut node_map = HashMap::with_capacity(descriptions.len());
        let mut children = HashSet::with_capacity(descriptions.len());
        for description in descriptions {
            let (parent_value, child_value, is_left) =
                (description[0], description[1], description[2] == 1);
            node_map
                .entry(parent_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent_value))));
            node_map
                .entry(child_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child_value))));
            if is_left {
                node_map.get(&parent_value).unwrap().borrow_mut().left =
                    Some(node_map.get(&child_value).unwrap().clone());
            } else {
                node_map.get(&parent_value).unwrap().borrow_mut().right =
                    Some(node_map.get(&child_value).unwrap().clone());
            }
            children.insert(child_value);
        }
        for node in node_map.values() {
            if !children.contains(&node.borrow().val) {
                return Some(node.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];
        let tree = TreeNode::from(vec![
            Some(50),
            Some(20),
            Some(80),
            Some(15),
            Some(17),
            Some(19),
        ]);
        let output = Solution::create_binary_tree(descriptions);
        assert_eq!(output, tree);
    }

    #[test]
    fn case_2() {
        let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];
        let tree = TreeNode::from(vec![Some(1), Some(2), None, None, Some(3), Some(4)]);
        assert_eq!(Solution::create_binary_tree(descriptions), tree);
    }
}
