/*

Given the head of a linked list, remove the nth node from the end of the list and return its head.

Example 1:

Input: head = [1,2,3,4,5], n = 2
Output: [1,2,3,5]
Example 2:

Input: head = [1], n = 1
Output: []
Example 3:

Input: head = [1,2], n = 1
Output: [1]

*/

// should work - should test...

use super::*;

fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut curr = head.clone();
    let mut list_len = 0;
    while let Some(curr_node) = curr {
        list_len += 1;
        curr = curr_node.next;
    }

    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head.clone();
    let mut til_remove = list_len - n;
    while let Some(curr_node) = curr.clone() {
        if til_remove == 0 {
            if prev.is_none() {
                // removing head
                head = head.unwrap().next;
                break;
            } else if curr_node.next.is_none() {
                // removing tail
                prev.unwrap().next = None;
                break;
            } else {
                // in the middle
                prev.unwrap().next = curr_node.next;
                break;
            }
        }

        prev = curr;
        curr = curr_node.next;
        til_remove -= 1;
    }
    return head;
}
