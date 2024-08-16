use linked::ListNode;

pub struct Solution;

impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let (mut slow, mut fast) = (&head, &head);
        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        let (mut prev, mut cur) = (None, slow.clone());
        while let Some(mut node) = cur.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            cur = next;
        }
        let (mut back, mut max_sum) = (prev, i32::MIN);
        while let Some(mut b_node) = back.take() {
            let mut f_node = head.unwrap();
            max_sum = max_sum.max(f_node.val + b_node.val);
            (head, back) = (f_node.next.take(), b_node.next.take());
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[5, 4, 2, 1]);
        assert_eq!(6, Solution::pair_sum(head));
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[4, 2, 2, 3]);
        assert_eq!(7, Solution::pair_sum(head));
    }

    #[test]
    fn case_3() {
        let head = ListNode::from(&[1, 100_000]);
        assert_eq!(100_001, Solution::pair_sum(head));
    }
}
