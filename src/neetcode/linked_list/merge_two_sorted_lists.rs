/*
You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

Example 1:

Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Example 2:

Input: list1 = [], list2 = []
Output: []
Example 3:

Input: list1 = [], list2 = [0]
Output: [0]
*/

// FIXME: look at the answer for this one
/*

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists_helper(
    list1: &mut Option<Box<ListNode>>,
    list2: &mut Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    return match (list1, list2) {
        (None, None) => None,
        (Some(h), None) | (None, Some(h)) => Some(h.clone()),
        (Some(l1), Some(l2)) => {
            if l1.val <= l2.val {
                Some(l1.clone())
            } else {
                Some(l2.clone())
            }
        }
    };
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    return match (list1, list2) {
        (None, None) => None,
        (Some(h), None) | (None, Some(h)) => {
            // only one list - which is guaranteed to be sorted already
            return Some(h);
        }
        (Some(l1), Some(l2)) => {
            let mut head = if l1.val <= l2.val {
                let temp = l1.clone();
                list1 = l1.next;
                temp
            } else {
                let temp = l2.clone();
                list2 = l2.next;
                temp
            };
            // head.next = merge_two_lists(list1, list2);
            return Some(head);
        } // (Some(mut h1), Some(mut h2)) => {
          //     let head = {
          //         if h1.val <= h2.val {
          //             h1.clone()
          //             // h1 = h1.next;
          //         } else {
          //             h2.clone()
          //         }
          //     };

          //     // while let (Some(first), Some(second)) = (&h1, &h2) {

          //     // };

          //     // while let

          //     // let head = first.clone();

          //     // one of the two linked lists ran out of elements - append the non-null one

          //     // while let Some(next) = res.unwrap().next && next

          //     return Some(head);
          // }
    };
}

*/
