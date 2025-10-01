//! You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: l1 = [2,4,3], l2 = [5,6,4]
//! Output: [7,0,8]
//! Explanation: 342 + 465 = 807.
//! Example 2:
//!
//! Input: l1 = [0], l2 = [0]
//! Output: [0]
//! Example 3:
//!
//! Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//! Output: [8,9,9,9,0,0,0,1]
//!  
//!
//! Constraints:
//!
//! The number of nodes in each linked list is in the range [1, 100].
//! 0 <= Node.val <= 9
//! It is guaranteed that the list represents a number that does not have leading zeros.

use std::ops::Deref;

use super::*;

// Definition for singly-linked list.
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

impl From<Vec<i32>> for ListNode {
    fn from(value: Vec<i32>) -> Self {
        let mut value = value.into_iter();
        let mut out = Some(Box::new(Self::new(value.next().unwrap())));
        for v in value {
            let next = Self::new(v);
            Solution::update_output(Box::new(next), &mut out);
        }
        *out.unwrap()
    }
}

struct IterNode(Option<Box<ListNode>>);

impl Deref for IterNode {
    type Target = Option<Box<ListNode>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for IterNode {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let Some(inner) = &mut self.0 else {
            return None;
        };
        let out = Some(inner.val);
        self.0 = inner.next.take();
        out
    }
}

impl Solution {
    pub fn update_output(v: Box<ListNode>, out: &mut Option<Box<ListNode>>) {
        let Some(inner) = out else {
            *out = Some(v);
            return;
        };
        Self::update_output(v, &mut inner.next);
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut add_one = false;
        let mut out = None;
        let mut l1 = IterNode(l1);
        let mut l2 = IterNode(l2);

        while l1.is_some() || l2.is_some() || add_one {
            let val =
                l1.next().unwrap_or_default() + l2.next().unwrap_or_default() + i32::from(add_one);
            add_one = val >= 10;

            Self::update_output(Box::new(ListNode::new(val % 10)), &mut out);
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2() {
        let l1 = vec![2, 4, 3];
        let l1 = ListNode::from(l1);
        let l2 = vec![5, 6, 4];
        let l2 = ListNode::from(l2);
        let out = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        let r = vec![7, 0, 8];
        let r = ListNode::from(r);
        debug_assert_eq!(out, Some(Box::new(r)));

        let l1 = vec![0];
        let l1 = ListNode::from(l1);
        let l2 = vec![0];
        let l2 = ListNode::from(l2);
        let out = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        let r = vec![0];
        let r = ListNode::from(r);
        debug_assert_eq!(out, Some(Box::new(r)));

        let l1 = vec![9, 9, 9, 9, 9, 9, 9];
        let l1 = ListNode::from(l1);
        let l2 = vec![9, 9, 9, 9];
        let l2 = ListNode::from(l2);
        let out = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        let r = vec![8, 9, 9, 9, 0, 0, 0, 1];
        let r = ListNode::from(r);
        debug_assert_eq!(out, Some(Box::new(r)));
    }
}
