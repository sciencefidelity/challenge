use b_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn lowest_common_ancestor(root: Tree, _p: Tree, _q: Tree) -> Tree {
        //     let n = 30;
        //     let mut nodes = vec![0; n];
        //     let mut depths = vec![0; n];
        //     let mut last = vec![0; n];
        //     let mut tour_index = 0;
        //     let mut node_index = 0;
        //
        //     Self::dfs(
        //         &root,
        //         0,
        //         &mut nodes,
        //         &mut depths,
        //         &mut last,
        //         &mut node_index,
        //         &mut tour_index,
        //     );
        //     println!("nodes: {nodes:?}");
        //     println!("depths: {depths:?}");
        //     println!("last: {last:?}");
        //     root
        // }
        //
        // fn dfs(
        //     maybe_node: &Tree,
        //     depth: usize,
        //     nodes: &mut Vec<i32>,
        //     depths: &mut Vec<usize>,
        //     last: &mut Vec<usize>,
        //     node_index: &mut usize,
        //     tour_index: &mut usize,
        // ) {
        //     if let Some(node) = maybe_node.as_ref() {
        //         let node = node.borrow();
        //         Self::visit(node.val, depth, nodes, depths, tour_index);
        //         Self::dfs(
        //             &node.left,
        //             depth + 1,
        //             nodes,
        //             depths,
        //             last,
        //             node_index,
        //             tour_index,
        //         );
        //         // *node_index += 1;
        //         // match last.get_mut(*node_index) {
        //         //     Some(idx) => *idx = tour_index,
        //         //     None => last.push(tour_index),
        //         // }
        //         Self::dfs(
        //             &node.right,
        //             depth + 1,
        //             nodes,
        //             depths,
        //             last,
        //             node_index,
        //             tour_index,
        //         );
        //         Self::visit(node.val, depth, nodes, depths, tour_index);
        //         Self::visit(node.val, depth, nodes, depths, tour_index);
        // }
        root
    }

    // fn visit(
    //     node_val: i32,
    //     depth: usize,
    //     nodes: &mut Vec<i32>,
    //     depths: &mut Vec<usize>,
    //     tour_index: &mut usize,
    // ) {
    //     println!("{tour_index}");
    //     nodes[*tour_index] = node_val;
    //     depths[*tour_index] = depth;
    //     *tour_index += 1;
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn case_1() {
//         let root = TreeNode::from(vec![
//             Some(3),
//             Some(5),
//             Some(1),
//             Some(6),
//             Some(2),
//             Some(0),
//             Some(8),
//             None,
//             None,
//             Some(7),
//             Some(4),
//         ]);
//         let p = TreeNode::from(vec![
//             Some(5),
//             Some(6),
//             Some(2),
//             None,
//             None,
//             Some(7),
//             Some(4),
//         ]);
//         let q = TreeNode::from(vec![Some(1), Some(0), Some(8)]);
//         let expected = TreeNode::from(vec![
//             Some(3),
//             Some(5),
//             Some(1),
//             Some(6),
//             Some(2),
//             Some(0),
//             Some(8),
//             None,
//             None,
//             Some(7),
//             Some(4),
//         ]);
//         Solution::lowest_common_ancestor(root, p, q);
//         assert!(false);
//         // assert_eq!(expected, Solution::lowest_common_ancestor(root, p, q));
//     }
//
//     #[test]
//     fn case_2() {
//         let root = TreeNode::from(vec![
//             Some(3),
//             Some(5),
//             Some(1),
//             Some(6),
//             Some(2),
//             Some(0),
//             Some(8),
//             None,
//             None,
//             Some(7),
//             Some(4),
//         ]);
//         let p = TreeNode::from(vec![
//             Some(5),
//             Some(6),
//             Some(2),
//             None,
//             None,
//             Some(7),
//             Some(4),
//         ]);
//         let q = TreeNode::from(vec![Some(4)]);
//         let expected = TreeNode::from(vec![
//             Some(5),
//             Some(6),
//             Some(2),
//             None,
//             None,
//             Some(7),
//             Some(4),
//         ]);
//         Solution::lowest_common_ancestor(root, p, q);
//         assert!(false);
//         // assert_eq!(expected, Solution::lowest_common_ancestor(root, p, q));
//     }
// }
