impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut sum = 0;
        let mut carry = 0;
        let mut head = l3.as_mut();
        let (mut l1, mut l2) = (&l1, &l2);

        while l1.is_some() || l2.is_some() {
            sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next;
            }
            carry = sum / 10;
            head.as_mut().unwrap().next = Some(Box::new(ListNode { val: sum % 10, next: None }));
            head = head.unwrap().next.as_mut();
        }
        if carry > 0 {
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        l3.unwrap().next
    }
}
