/// ListNode: a singly linked list implementation based on requirements of Leet Code
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    /// Creates a new empty `ListNode`
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// Creates a list from a vector of integers
    pub fn from(vec: Vec<i32>) -> Option<Box<Self>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }
}
