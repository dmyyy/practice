use std::mem;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = reverse_list(l1);
    let mut l2 = reverse_list(l2);

    let mut out = None;
    let mut cur = &mut out;

    let mut carry = 0;
    
    while l1.is_some() || l2.is_some() || carry > 0 {
        let a = l1.as_ref().map(|x| x.val).unwrap_or(0);
        let b = l2.as_ref().map(|x| x.val).unwrap_or(0);
        
        let s = a + b + carry;
        carry = s / 10;
        
        let next_node = Box::new(ListNode::new(s % 10));
        cur = &mut cur.insert(next_node).next;
        
        l1 = l1.and_then(|x| x.next);
        l2 = l2.and_then(|x| x.next);
    }
            
    reverse_list(out)
}

fn reverse_list(l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = l;
    let mut head = None;

    while let Some(mut n) = cur {
        cur = mem::replace(&mut n.next, head);
        head = Some(n);
    }

    head
}