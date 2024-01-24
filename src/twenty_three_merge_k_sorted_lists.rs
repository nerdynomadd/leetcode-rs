use std::cmp::Ordering;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..lists.len() {
            let mut node = lists[i].take();
            while let Some(ref n) = node {
                heap.push(n.val);
                node = node.unwrap().next;
            }
        }
        let mut head = None;
        let mut tail = &mut head;
        for i in heap.into_sorted_vec() {
            *tail = Some(Box::new(ListNode::new(i)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }
}

impl std::cmp::PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl std::cmp::Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}