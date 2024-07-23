use linked::ListNode;

pub struct Solution;

type List = Option<Box<ListNode>>;

impl Solution {
    pub fn delete_duplicates(mut head: List) -> List {
        let (mut ret, mut to_remove) = (None, i32::MAX);
        let mut tail = &mut ret;
        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            if node.val == to_remove {
                continue;
            }
            if head.is_some() && node.val == head.as_ref().unwrap().val {
                to_remove = node.val;
                continue;
            }
            tail = &mut tail.insert(node).next;
            to_remove = i32::MAX;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let list = ListNode::from(&[1, 2, 3, 3, 4, 4, 5]);
        let expected = ListNode::from(&[1, 2, 5]);
        assert_eq!(expected, Solution::delete_duplicates(list));
    }

    #[test]
    fn case_2() {
        let list = ListNode::from(&[1, 1, 1, 2, 3]);
        let expected = ListNode::from(&[2, 3]);
        assert_eq!(expected, Solution::delete_duplicates(list));
    }
}
