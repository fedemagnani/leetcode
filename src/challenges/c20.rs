//! Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//!
//! An input string is valid if:
//!
//! Open brackets must be closed by the same type of brackets.
//! Open brackets must be closed in the correct order.
//! Every close bracket has a corresponding open bracket of the same type.
//!  
//!
//! Example 1:
//!
//! Input: s = "()"
//!
//! Output: true
//!
//! Example 2:
//!
//! Input: s = "()[]{}"
//!
//! Output: true
//!
//! Example 3:
//!
//! Input: s = "(]"
//!
//! Output: false
//!
//! Example 4:
//!
//! Input: s = "([])"
//!
//! Output: true
//!
//! Example 5:
//!
//! Input: s = "([)]"
//!
//! Output: false
//!
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 104
//! s consists of parentheses only '()[]{}'.

use super::*;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.as_bytes();
        // LIFO
        let mut stack = Vec::with_capacity(s.len());
        for s in s {
            if s == &b')' {
                let Some(f) = stack.pop() else { return false };
                if f != b'(' {
                    return false;
                }
            } else if s == &b']' {
                let Some(f) = stack.pop() else { return false };
                if f != b'[' {
                    return false;
                }
            } else if s == &b'}' {
                let Some(f) = stack.pop() else { return false };
                if f != b'{' {
                    return false;
                }
            } else {
                stack.push(*s);
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test20() {
        let s = "()";
        let out = Solution::is_valid(s.to_string());
        debug_assert!(out);

        let s = "()[]{}";
        let out = Solution::is_valid(s.to_string());
        debug_assert!(out);

        let s = "(]";
        let out = Solution::is_valid(s.to_string());
        debug_assert!(!out);

        let s = "([])";
        let out = Solution::is_valid(s.to_string());
        debug_assert!(out);

        let s = "([)]";
        let out = Solution::is_valid(s.to_string());
        debug_assert!(!out);
    }

    #[test]
    fn t() {
        println!("{}", 91 / 41);
    }
}
