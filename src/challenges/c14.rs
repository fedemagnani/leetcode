//! Write a function to find the longest common prefix string amongst an array of strings.
//!
//! If there is no common prefix, return an empty string "".
//!
//!  
//!
//! Example 1:
//!
//! Input: strs = ["flower","flow","flight"]
//! Output: "fl"
//! Example 2:
//!
//! Input: strs = ["dog","racecar","car"]
//! Output: ""
//! Explanation: There is no common prefix among the input strings.
//!  
//!
//! Constraints:
//!
//! 1 <= strs.length <= 200
//! 0 <= strs[i].length <= 200
//! strs[i] consists of only lowercase English letters if it is non-empty.

use super::*;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let last = strs.pop().unwrap();
        let mut out = Vec::with_capacity(last.len());

        let mut strs = strs
            .into_iter()
            .map(|x| x.into_bytes().into_iter())
            .collect::<Vec<_>>();

        'main: for c in last.into_bytes() {
            for s in &mut strs {
                let Some(s_c) = s.next() else {
                    break 'main;
                };
                if s_c != c {
                    break 'main;
                }
            }
            out.push(c);
        }

        String::from_utf8(out).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test14() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let out = Solution::longest_common_prefix(strs);
        debug_assert_eq!(out, "fl");

        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let out = Solution::longest_common_prefix(strs);
        debug_assert_eq!(out, "");
    }
}
