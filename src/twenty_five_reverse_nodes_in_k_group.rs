impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut flatened = Vec::new();
        let mut head = head;
        while let Some(node) = head {
            flatened.push(node.val);
            head = node.next;
        }
        // Swap elements in k groups in the flatened list
        let mut i = 0;
        while i + k as usize <= flatened.len() {
            flatened[i..i + k as usize].reverse();
            i += k as usize;
        }
        // Reconstruct the list
        let mut head = None;
        for &val in flatened.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}