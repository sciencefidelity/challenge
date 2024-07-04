pub struct Solution;

use linked::ListNode;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_nodes(mut head: List) -> List {
        let Some(modify) = head.as_mut() else {
            return head;
        };
        let mut next_sum = &mut modify.next;
        while let Some(curr_node) = next_sum {
            let Some(next_node) = curr_node.next.as_mut() else {
                curr_node.next = None;
                break;
            };
            if next_node.val > 0 {
                curr_node.val += next_node.val;
                curr_node.next = next_node.next.take()
            } else {
                curr_node.next = next_node.next.take();
                next_sum = &mut next_sum.as_mut().unwrap().next
            }
        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = ListNode::from(vec![0, 3, 1, 0, 4, 5, 2, 0]);
        let l2 = ListNode::from(vec![4, 11]);
        let l1 = Solution::merge_nodes(l1);
        assert_eq!(l1, l2);
    }

    #[test]
    fn case_2() {
        let l1 = ListNode::from(vec![0, 1, 0, 3, 0, 2, 2, 0]);
        let l2 = ListNode::from(vec![1, 3, 4]);
        let l1 = Solution::merge_nodes(l1);
        assert_eq!(l1, l2);
    }
}
