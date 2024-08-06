use linked::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_mut().is_some() {
            let mut h = head;
            let mut p1 = h.as_mut().unwrap();
            while let Some(p2) = p1.next.as_mut() {
                if p1.val == p2.val {
                    p1.next = p2.next.take();
                } else {
                    p1 = p1.next.as_mut().unwrap();
                }
            }
            h
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let list = ListNode::from(&[1, 1, 2]);
        let expected = ListNode::from(&[1, 2]);
        assert_eq!(expected, Solution::delete_duplicates(list));
    }

    #[test]
    fn case_2() {
        let list = ListNode::from(&[1, 1, 2, 3, 3]);
        let expected = ListNode::from(&[1, 2, 3]);
        assert_eq!(expected, Solution::delete_duplicates(list));
    }
}
