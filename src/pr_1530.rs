use b_tree::TreeNode;
use std::cell::RefCell;
// use std::collections::{HashMap, HashSet, VecDeque};
// use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct Solution;

type Tree = Rc<RefCell<TreeNode>>;

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct NodeRc(Tree);
//
// impl Hash for NodeRc {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         Rc::as_ptr(&self.0).hash(state);
//     }
// }
//
// impl Solution {
//     pub fn count_pairs(root: Option<Tree>, distance: i32) -> i32 {
//         let mut graph = HashMap::new();
//         let mut leaf_nodes = HashSet::new();
//         Self::traverse_tree(root, None, &mut graph, &mut leaf_nodes);
//         let mut ans = 0;
//         for leaf in &leaf_nodes {
//             let mut bfs_queue = VecDeque::new();
//             let mut seen = HashSet::new();
//             bfs_queue.push_back(leaf);
//             seen.insert(leaf);
//             for _ in 0..=distance {
//                 let size = bfs_queue.len();
//                 for _ in 0..size {
//                     let curr_node = bfs_queue.pop_front().unwrap();
//                     if leaf_nodes.contains(curr_node) && curr_node != leaf {
//                         ans += 1;
//                     }
//                     if let Some(curr_node) = graph.get(curr_node) {
//                         for neighbor in curr_node {
//                             if !seen.contains(neighbor) {
//                                 bfs_queue.push_back(neighbor);
//                                 seen.insert(neighbor);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         ans / 2
//     }
//
//     fn traverse_tree(
//         curr_node: Option<Tree>,
//         prev_node: Option<Tree>,
//         graph: &mut HashMap<NodeRc, Vec<NodeRc>>,
//         leaf_nodes: &mut HashSet<NodeRc>,
//     ) {
//         if let Some(curr_node) = curr_node {
//             let curr_node_ref = curr_node.borrow();
//             if curr_node_ref.left.is_none() && curr_node_ref.right.is_none() {
//                 leaf_nodes.insert(NodeRc(curr_node.clone()));
//             }
//             if let Some(prev_node) = prev_node {
//                 graph
//                     .entry(NodeRc(prev_node.clone()))
//                     .or_default()
//                     .push(NodeRc(curr_node.clone()));
//                 graph
//                     .entry(NodeRc(curr_node.clone()))
//                     .or_default()
//                     .push(NodeRc(prev_node));
//             }
//             Self::traverse_tree(
//                 curr_node_ref.left.clone(),
//                 Some(curr_node.clone()),
//                 graph,
//                 leaf_nodes,
//             );
//             Self::traverse_tree(
//                 curr_node_ref.right.clone(),
//                 Some(curr_node.clone()),
//                 graph,
//                 leaf_nodes,
//             );
//         }
//     }
// }

impl Solution {
    pub fn count_pairs(root: Option<Tree>, distance: i32) -> i32 {
        Self::post_order(root, distance)[11]
    }

    fn post_order(current_node: Option<Tree>, distance: i32) -> Vec<i32> {
        if let Some(current_node) = current_node {
            let curr_node_ref = current_node.borrow();
            if curr_node_ref.left.is_none() && curr_node_ref.right.is_none() {
                let mut current = vec![0; 12];
                current[0] = 1;
                return current;
            }
            let left = Self::post_order(curr_node_ref.left.clone(), distance);
            let right = Self::post_order(curr_node_ref.right.clone(), distance);
            let mut current = vec![0; 12];
            for i in 0..10 {
                current[i + 1] += left[i] + right[i];
            }
            current[11] += left[11] + right[11];
            for d1 in 0..=distance {
                for d2 in 0..=distance {
                    if 2 + d1 + d2 <= distance {
                        current[11] += left[usize::try_from(d1).unwrap()]
                            * right[usize::try_from(d2).unwrap()];
                    }
                }
            }
            current
        } else {
            vec![0; 12]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let tree = TreeNode::from(vec![Some(1), Some(2), Some(3), None, Some(4)]);
        assert_eq!(Solution::count_pairs(tree, 3), 1);
    }

    #[test]
    fn case_2() {
        let tree = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        assert_eq!(Solution::count_pairs(tree, 3), 2);
    }

    #[test]
    fn case_3() {
        let tree = TreeNode::from(vec![
            Some(7),
            Some(1),
            Some(4),
            Some(6),
            None,
            Some(5),
            Some(3),
            None,
            None,
            None,
            None,
            None,
            Some(2),
        ]);
        assert_eq!(Solution::count_pairs(tree, 3), 1);
    }
}
