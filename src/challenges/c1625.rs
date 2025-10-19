//! You are given a string s of even length consisting of digits from 0 to 9, and two integers a and b.
//!
//! You can apply either of the following two operations any number of times and in any order on s:
//!
//! Add a to all odd indices of s (0-indexed). Digits post 9 are cycled back to 0. For example, if s = "3456" and a = 5, s becomes "3951".
//! Rotate s to the right by b positions. For example, if s = "3456" and b = 1, s becomes "6345".
//! Return the lexicographically smallest string you can obtain by applying the above operations any number of times on s.
//!
//! A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ, string a has a letter that appears earlier in the alphabet than the corresponding letter in b. For example, "0158" is lexicographically smaller than "0190" because the first position they differ is at the third letter, and '5' comes before '9'.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "5525", a = 9, b = 2
//! Output: "2050"
//! Explanation: We can apply the following operations:
//! Start:  "5525"
//! Rotate: "2555"
//! Add:    "2454"
//! Add:    "2353"
//! Rotate: "5323"
//! Add:    "5222"
//! Add:    "5121"
//! Rotate: "2151"
//! Add:    "2050"​​​​​
//! There is no way to obtain a string that is lexicographically smaller than "2050".
//! Example 2:
//!
//! Input: s = "74", a = 5, b = 1
//! Output: "24"
//! Explanation: We can apply the following operations:
//! Start:  "74"
//! Rotate: "47"
//! ​​​​​​​Add:    "42"
//! ​​​​​​​Rotate: "24"​​​​​​​​​​​​
//! There is no way to obtain a string that is lexicographically smaller than "24".
//! Example 3:
//!
//! Input: s = "0011", a = 4, b = 2
//! Output: "0011"
//! Explanation: There are no sequence of operations that will give us a lexicographically smaller string than "0011".
//!  
//!
//! Constraints:
//!
//! 2 <= s.length <= 100
//! s.length is even.
//! s consists of digits from 0 to 9 only.
//! 1 <= a <= 9
//! 1 <= b <= s.length - 1

// Expanding this problem you realize that you can build a fully cyclic graph: https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/solutions/7284462/it-s-a-graph-problem-yessir-let-s-draw-it-cycle-visualization-and-dfs/
// So a depth-first-search algorithm could help us solving the problem

use super::*;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let a = a as u8;
        let b = b as usize;
        let c = CircularBuffer::from(s.into_bytes());

        let mut stack = Stack::default();
        stack.push(c);
        while let Some(c) = stack.inner.pop() {
            let mut c1 = c.clone();
            let mut c2 = c.clone();
            c1.increase_odds(a);
            c2.rotate_right(b);
            stack.push(c1);
            stack.push(c2);
        }

        stack.best
    }
}

#[derive(Default)]
struct Stack {
    inner: Vec<CircularBuffer>,
    visited: std::collections::HashSet<String>,
    best: String,
}

impl Stack {
    fn push(&mut self, c: CircularBuffer) {
        let s = c.to_string();
        if self.best.is_empty() || s < self.best {
            self.best = s.clone()
        }
        if !self.visited.insert(s) {
            return;
        }
        self.inner.push(c);
    }
}

#[derive(Clone)]
struct CircularBuffer {
    inner: Vec<u8>,
    head: usize,
}

impl From<Vec<u8>> for CircularBuffer {
    fn from(inner: Vec<u8>) -> Self {
        Self { inner, head: 0 }
    }
}

impl CircularBuffer {
    fn rotate_right(&mut self, pos: usize) {
        let n = self.inner.len();
        // We don't move the array. Simply edit the logical index
        self.head = (self.head + n - (pos % n)) % n;
    }

    fn increase_odds(&mut self, by: u8) {
        let n = self.inner.len();

        for i in 0..n {
            if i.is_multiple_of(2) {
                continue;
            }
            let i = (self.head + i) % n;
            self.inner[i] -= b'0';
            self.inner[i] += by;
            self.inner[i] %= 10;
            self.inner[i] += b'0';
        }
    }
}

impl std::fmt::Display for CircularBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.inner.len();
        for i in 0..n {
            let i = (self.head + i) % n; //phisical index
            let v = self.inner[i] - b'0'; //utf8 decode for numbers
            write!(f, "{v}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::string::FromUtf8Error;

    use super::*;

    #[test]
    fn test1625() {
        let s = "5525";
        let a = 9;
        let b = 2;
        let exp = "2050";
        let out = Solution::find_lex_smallest_string(s.to_string(), a, b);
        debug_assert_eq!(exp, out);

        let s = "74";
        let a = 5;
        let b = 1;
        let exp = "24";
        let out = Solution::find_lex_smallest_string(s.to_string(), a, b);
        debug_assert_eq!(exp, out);

        let s = "0011";
        let a = 4;
        let b = 2;
        let exp = "0011";
        let out = Solution::find_lex_smallest_string(s.to_string(), a, b);
        debug_assert_eq!(exp, out);
    }

    #[test]
    fn test_case() {
        let s = "5525";
        let a = 9_u8;
        let b = 2;
        let exp = "2050";

        let mut c = CircularBuffer::from(s.to_owned().into_bytes());
        c.rotate_right(b);
        debug_assert_eq!(c.to_string(), "2555");

        c.increase_odds(a);
        debug_assert_eq!(c.to_string(), "2454");

        c.increase_odds(a);
        debug_assert_eq!(c.to_string(), "2353");

        c.rotate_right(b);
        debug_assert_eq!(c.to_string(), "5323");

        c.increase_odds(a);
        debug_assert_eq!(c.to_string(), "5222");

        c.increase_odds(a);
        debug_assert_eq!(c.to_string(), "5121");

        c.rotate_right(b);
        debug_assert_eq!(c.to_string(), "2151");

        c.increase_odds(a);
        debug_assert_eq!(c.to_string(), exp);
    }

    #[test]
    fn bytes_range() {
        debug_assert_eq!(9, b'9' - b'0');
    }

    #[test]
    fn overflow_sum() -> Result<(), FromUtf8Error> {
        let s = "13456".to_string();
        let add_am = 9;

        let mut chars = s.into_bytes();

        for (i, c) in chars.iter_mut().enumerate() {
            let odd = i.is_multiple_of(2);
            if odd {
                *c -= b'0';
                *c += add_am;
                *c %= 10;
                *c += b'0';
            }
        }

        let out = String::from_utf8(chars)?;

        debug_assert_eq!(out, "03355");

        Ok(())
    }

    #[test]
    fn circular_buffer() {
        let s = "13456".to_string();
        let mut c = CircularBuffer::from(s.into_bytes());
        debug_assert_eq!(c.to_string(), "13456");

        c.rotate_right(2);
        debug_assert_eq!(c.to_string(), "56134");

        c.increase_odds(9);
        debug_assert_eq!(c.to_string(), "55124");
    }
}
