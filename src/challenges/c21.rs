//! You are given the heads of two sorted linked lists list1 and list2.
//!
//! Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
//!
//! Return the head of the merged linked list.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: list1 = [1,2,4], list2 = [1,3,4]
//! Output: [1,1,2,3,4,4]
//! Example 2:
//!
//! Input: list1 = [], list2 = []
//! Output: []
//! Example 3:
//!
//! Input: list1 = [], list2 = [0]
//! Output: [0]
//!  
//!
//! Constraints:
//!
//! The number of nodes in both lists is in the range [0, 50].
//! -100 <= Node.val <= 100
//! Both list1 and list2 are sorted in non-decreasing order.

use super::*;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut traveller = &mut list1;
        let reserve = &mut list2;

        // continue until there is some reserve still
        while let Some(r) = reserve {
            // swap roles if traveller is exhausted or if the reserve is better than the travel
            if traveller.is_none() || r.val < traveller.as_ref()?.val {
                // swap roles
                std::mem::swap(traveller, reserve);
            }
            // advance the traveller: not iw poins to the deepest part of the chain
            traveller = &mut traveller.as_mut()?.next
        }
        list1
    }
}
