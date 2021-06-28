use std::collections::VecDeque;
// Definition for singly-linked list.
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
}

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut window: VecDeque<Box<ListNode>> = VecDeque::with_capacity((n+1) as usize);
        let mut new_head = None;
        let mut new_node = &mut new_head;

        while let Some(mut node) = head {
            head = node.next.take();

            window.push_back(node);

            while window.len() > n as usize {
                let pending_node = window.pop_front().unwrap();
                *new_node = Some(pending_node);
                new_node = &mut new_node.as_mut().unwrap().next;
            }
        }

        window.pop_front(); //remove nth node

        for pending_node in window {
            *new_node = Some(pending_node);
            new_node = &mut new_node.as_mut().unwrap().next;
        }

        new_head
    }
}