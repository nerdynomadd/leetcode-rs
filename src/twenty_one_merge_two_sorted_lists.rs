#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        while let (Some(l1), Some(l2)) = (&list1, &list2) {
            if l1.val < l2.val {
                tail.next = list1;
                list1 = tail.next.as_mut().unwrap().next.take();
            } else {
                tail.next = list2;
                list2 = tail.next.as_mut().unwrap().next.take();
            }

            tail = tail.next.as_mut().unwrap();
        }

        tail.next = list1.or(list2);
        head.next
    }
}