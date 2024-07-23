use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn partition(mut head: List, x: i32) -> List {
        let (mut node1, mut node2) = (ListNode::new(0), ListNode::new(0));
        let (mut p1, mut p2) = (&mut node1, &mut node2);
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                p1.next = Some(node);
                p1 = p1.next.as_mut().unwrap();
            } else {
                p2.next = Some(node);
                p2 = p2.next.as_mut().unwrap();
            }
        }
        p1.next = node2.next;
        node1.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let list = ListNode::from(&[1, 4, 3, 2, 5, 2]);
        let expected = ListNode::from(&[1, 2, 2, 4, 3, 5]);
        assert_eq!(expected, Solution::partition(list, 3));
    }

    #[test]
    fn case_2() {
        let list = ListNode::from(&[2, 1]);
        let expected = ListNode::from(&[1, 2]);
        assert_eq!(expected, Solution::partition(list, 2));
    }
}
