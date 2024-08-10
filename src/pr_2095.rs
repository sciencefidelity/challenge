use linked::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut fast = &(*dummy) as *const ListNode;
        let mut slow = &mut (*dummy) as *mut ListNode;
        unsafe {
            while (*fast).next.is_some() && (*fast).next.as_deref().unwrap().next.is_some() {
                fast = (*fast).next.as_deref().unwrap().next.as_deref().unwrap();
                slow = (*slow).next.as_deref_mut().unwrap();
            }
            let to_delete = (*slow).next.take();
            (*slow).next = to_delete.unwrap().next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[1, 3, 4, 7, 1, 2, 6]);
        let expected = ListNode::from(&[1, 3, 4, 1, 2, 6]);
        assert_eq!(expected, Solution::delete_middle(head));
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[1, 2, 3, 4]);
        let expected = ListNode::from(&[1, 2, 4]);
        assert_eq!(expected, Solution::delete_middle(head));
    }

    #[test]
    fn case_3() {
        let head = ListNode::from(&[2, 1]);
        let expected = ListNode::from(&[2]);
        assert_eq!(expected, Solution::delete_middle(head));
    }
}
