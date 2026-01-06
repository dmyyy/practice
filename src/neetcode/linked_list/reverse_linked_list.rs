// list node needs to be boxed otherwise won't be able to figure out size

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// IMPORTANT: for any rust linked list problem - we need an explicit pointer to the front of the list...
// - it should NOT be a ListNode (this Option<Box<T>> is the actual data type we're going to be moving around
// as a pointer in the list)

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<ListNode>>,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None }
    }

    // push to the front of the list
    fn push_front(&mut self, val: i32) {
        let new_head = ListNode {
            val,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_head));
    }

    fn pop_front(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.val)
            }
            None => None,
        }
    }

    // remove the last value in the list
    fn pop_back(&mut self) -> Option<i32> {
        while let Some(curr) = self.head.take() {
            if curr.next.is_none() {
                // this is the last node - remove it!
                // I can't remove it unless I keep track of the previous/back pointer?
                let result = Some(curr.val);
                drop(curr);
                return result;
            }
        }
        None
    }

    fn reverse(&mut self) {
        let mut curr_head = self.head.take();
        while let Some(mut curr_node) = curr_head {
            curr_head = curr_node.next.take();
            curr_node.next = self.head.take();
            self.head = Some(curr_node);
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut node) = curr {
            curr = node.next.take();
        }
    }
}

impl Into<Vec<i32>> for LinkedList {
    fn into(mut self) -> Vec<i32> {
        let mut items = Vec::new();
        while let Some(curr) = self.head.take() {
            items.push(curr.val);
            self.head = curr.next;
        }

        items
    }
}

// fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}

// i think doing linked list and tree stuff in rust is probably a

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let mut linked_list = LinkedList::new();
        linked_list.push_front(1);
        linked_list.push_front(2);
        linked_list.push_front(3);
        linked_list.push_front(4);
        linked_list.push_front(5);

        let mut items: Vec<i32> = linked_list.into();
        assert_eq!(items, vec![5, 4, 3, 2, 1]);
    }
}
