use linked::ListNode;

pub struct Solution;

type OptNode = Option<Box<ListNode>>;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: OptNode) -> OptNode {
        let mut map = vec![false; 100_001];
        for n in nums {
            map[usize::try_from(n).unwrap()] = true;
        }
        let mut root = ListNode { val: 0, next: head };
        let mut curr = &mut root;
        while let Some(next) = curr.next.as_mut() {
            if map[usize::try_from(next.val).unwrap()] {
                curr.next = next.next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }
        root.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let head = ListNode::from(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from(&[4, 5]);
        assert_eq!(Solution::modified_list(vec![1, 2, 3], head), expected);
    }

    #[test]
    fn case_2() {
        let head = ListNode::from(&[1, 2, 1, 2, 1, 2]);
        let expected = ListNode::from(&[2, 2, 2]);
        assert_eq!(Solution::modified_list(vec![1], head), expected);
    }

    #[test]
    fn case_3() {
        let head = ListNode::from(&[1, 2, 3, 4]);
        let expected = ListNode::from(&[1, 2, 3, 4]);
        assert_eq!(Solution::modified_list(vec![5], head), expected);
    }
}
