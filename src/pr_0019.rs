use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn remove_nth_from_end(head: List, n: i32) -> List {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut fast = &(*dummy) as *const ListNode;
        for _ in 0..n {
            unsafe {
                (*fast).next.as_ref()?;
                fast = (*fast).next.as_deref().unwrap();
            }
        }
        let mut slow = &mut (*dummy) as *mut ListNode;
        unsafe {
            while (*fast).next.is_some() {
                fast = (*fast).next.as_deref().unwrap();
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
        let list = ListNode::from(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from(&[1, 2, 3, 5]);
        assert_eq!(expected, Solution::remove_nth_from_end(list, 2));
    }

    #[test]
    fn case_2() {
        let list = ListNode::from(&[1]);
        let expected = ListNode::from(&[]);
        assert_eq!(expected, Solution::remove_nth_from_end(list, 1));
    }

    #[test]
    fn case_3() {
        let list = ListNode::from(&[1, 2]);
        let expected = ListNode::from(&[1]);
        assert_eq!(expected, Solution::remove_nth_from_end(list, 1));
    }
}
