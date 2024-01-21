use std::collections::VecDeque;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut queue = VecDeque::with_capacity(n as usize + 1);
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy.next;

        while let Some(mut node) = head.take() {
            head = node.next.take();

            queue.push_back(node);
            if n == queue.len() as i32 - 1 {
                *tail = queue.pop_front();
                tail = &mut tail.as_mut().unwrap().next;
            }
        }

        queue.pop_front();

        while let Some(mut node) = queue.pop_front() {
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        dummy.next.take()
    }
}