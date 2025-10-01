//! Given a string s, find the length of the longest substring without duplicate characters.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "abcabcbb"
//! Output: 3
//! Explanation: The answer is "abc", with the length of 3.
//! Example 2:
//!
//! Input: s = "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//! Example 3:
//!
//! Input: s = "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//! Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//!  
//!
//! Constraints:
//!
//! 0 <= s.length <= 5 * 104
//! s consists of English letters, digits, symbols and spaces.

use super::*;

#[derive(Default, Debug)]
struct Substr {
    inner: Vec<char>,
    highest: usize,
}
impl Substr {
    fn append(&mut self, c: char) {
        let position = self.inner.iter().position(|x| x == &c);
        self.inner.push(c);
        let Some(i) = position else {
            let l = self.inner.len();
            if l > self.highest {
                self.highest = l;
            }
            return;
        };
        self.inner = self.inner[i + 1..].to_vec();
    }
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut out = Substr::default();
        for c in s.chars() {
            out.append(c);
        }
        out.highest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "abcabcbb";
        let out = Solution::length_of_longest_substring(s.to_string());
        debug_assert_eq!(out, 3);

        let s = "bbbbb";
        let out = Solution::length_of_longest_substring(s.to_string());
        debug_assert_eq!(out, 1);
        let s = "dvdf";

        let out = Solution::length_of_longest_substring(s.to_string());
        debug_assert_eq!(out, 3);

        let s = "pwwkew";
        let out = Solution::length_of_longest_substring(s.to_string());
        debug_assert_eq!(out, 3);

        let s = "anviaj";
        let out = Solution::length_of_longest_substring(s.to_string());
        debug_assert_eq!(out, 5);

        let s = "aabaab!bb";
        let out = Solution::length_of_longest_substring(s.to_string());
        debug_assert_eq!(out, 3);
    }
}
