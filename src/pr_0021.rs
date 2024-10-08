use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_two_lists(list1: List, list2: List) -> List {
        match (&list1, &list2) {
            (None, None) => None,
            (Some(_), None) => list1,
            (None, Some(_)) => list2,
            (Some(l), Some(r)) => {
                if l.val <= r.val {
                    Some(Box::new(ListNode {
                        val: l.val,
                        next: Self::merge_two_lists(l.next.clone(), list2),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: r.val,
                        next: Self::merge_two_lists(list1, r.next.clone()),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let l1 = ListNode::from(&[1, 2, 4]);
        let l2 = ListNode::from(&[1, 3, 4]);
        let list = ListNode::from(&[1, 1, 2, 3, 4, 4]);
        assert_eq!(list, Solution::merge_two_lists(l1, l2));
    }

    #[test]
    fn case_2() {
        let l1 = ListNode::from(&[]);
        let l2 = ListNode::from(&[]);
        let list = ListNode::from(&[]);
        assert_eq!(list, Solution::merge_two_lists(l1, l2));
    }

    #[test]
    fn case_3() {
        let l1 = ListNode::from(&[]);
        let l2 = ListNode::from(&[0]);
        let list = ListNode::from(&[0]);
        assert_eq!(list, Solution::merge_two_lists(l1, l2));
    }
}
