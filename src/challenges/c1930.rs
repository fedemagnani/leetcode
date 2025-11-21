//! Given a string s, return the number of unique palindromes of length three that are a subsequence of s.
//!
//! Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.
//!
//! A palindrome is a string that reads the same forwards and backwards.
//!
//! A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
//!
//! For example, "ace" is a subsequence of "abcde".
//!  
//!
//! Example 1:
//!
//! Input: s = "aabca"
//! Output: 3
//! Explanation: The 3 palindromic subsequences of length 3 are:
//! - "aba" (subsequence of "aabca")
//! - "aaa" (subsequence of "aabca")
//! - "aca" (subsequence of "aabca")
//!
//! Example 2:
//!
//! Input: s = "adc"
//! Output: 0
//! Explanation: There are no palindromic subsequences of length 3 in "adc".
//! Example 3:
//!
//! Input: s = "bbcbaba"
//! Output: 4
//! Explanation: The 4 palindromic subsequences of length 3 are:
//! - "bbb" (subsequence of "bbcbaba")
//! - "bcb" (subsequence of "bbcbaba")
//! - "bab" (subsequence of "bbcbaba")
//! - "aba" (subsequence of "bbcbaba")
//!  
//!
//! Constraints:
//!
//! 3 <= s.length <= 105
//! s consists of only lowercase English letters.

use super::*;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut first = [-1; 26];
        let mut last = [-1; 26];

        for (i, b) in bytes.iter().enumerate() {
            let c = (b - b'a') as usize;
            if first[c] == -1 {
                first[c] = i as i32;
            }
            last[c] = i as i32;
        }

        let mut ans = 0;

        for c in 0..26 {
            if first[c] != -1 && last[c] - first[c] > 1 {
                let mut mask = 0u32;
                let start = first[c] as usize + 1;
                let end = last[c] as usize;

                for b in bytes.iter().take(end).skip(start) {
                    let mid = (b - b'a') as usize;
                    mask |= 1 << mid;
                }

                ans += mask.count_ones() as i32;
            }
        }

        ans
    }
}
