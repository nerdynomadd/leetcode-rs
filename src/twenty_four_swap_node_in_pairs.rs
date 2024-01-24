impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut flatened = Vec::new();
        let mut head = head;
        while let Some(node) = head {
            flatened.push(node.val);
            head = node.next;
        }
        // Swap elements in pairs in the flatened list
        for i in (0..flatened.len()).step_by(2) {
            if i + 1 < flatened.len() {
                flatened.swap(i, i + 1);
            }
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