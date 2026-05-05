//! Given the head of a linked list, rotate the list to the right by k places.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: head = [1,2,3,4,5], k = 2
//! Output: [4,5,1,2,3]
//! Example 2:
//!
//!
//! Input: head = [0,1,2], k = 4
//! Output: [2,0,1]
//!  
//!
//! Constraints:
//!
//! The number of nodes in the list is in the range [0, 500].
//! -100 <= Node.val <= 100
//! 0 <= k <= 2 * 109

// Definition for singly-linked list.

use super::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut head = head?;

        let len = compute_list_length(&head);

        let k = k % len;
        if k == 0 {
            return Some(head);
        }

        // 2. Walk to the split point: (len - k - 1) steps
        let mut cur = &mut head;
        for _ in 0..(len - k - 1) {
            cur = cur.next.as_mut().unwrap();
        }

        // 3. Cut here: new_head is everything after cur
        let mut new_head = cur.next.take().unwrap();

        // 4. Walk new_head to its end and attach old head
        let mut tail = &mut new_head;
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = Some(head);

        Some(new_head)
    }
}

// O(n)
fn compute_list_length(head: &ListNode) -> i32 {
    let mut out = 1;
    let mut next = &head.next;
    while let Some(next_h) = next {
        next = &next_h.next;
        out += 1;
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    fn make_list<const N: usize>(list: [i32; N]) -> Box<ListNode> {
        let list = list.into_iter().rev().fold(None, |maybe_node, x| {
            let mut node = ListNode::new(x);
            node.next = maybe_node;
            Some(Box::new(node))
        });
        list.unwrap()
    }

    fn make_array(list: ListNode, length: usize) -> Vec<i32> {
        let mut out = Vec::with_capacity(length);
        out.push(list.val);
        let mut next = list.next;
        while let Some(mut list) = next {
            out.push(list.val);
            next = list.next.take()
        }
        out
    }

    #[test]
    fn test61() {
        let list_ar = [1, 2, 3, 4, 5];
        let list = make_list(list_ar);

        assert_eq!(compute_list_length(&list), 5);
        let new_list = make_array(*list.clone(), 5);

        assert_eq!(&new_list, &list_ar);

        let k = 2;

        let target_ar = [4, 5, 1, 2, 3];
        let target = make_list(target_ar);

        let rotated = Solution::rotate_right(Some(list), k).unwrap();

        assert_eq!(target, rotated)
    }
}
