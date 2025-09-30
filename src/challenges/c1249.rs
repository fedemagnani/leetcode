//! Given a string s of '(' , ')' and lowercase English characters.
//!
//! Your task is to remove the minimum number of parentheses ( '(' or ')', in any positions ) so that the resulting parentheses string is valid and return any valid string.
//!
//! Formally, a parentheses string is valid if and only if:
//!
//! It is the empty string, contains only lowercase characters, or
//! It can be written as AB (A concatenated with B), where A and B are valid strings, or
//! It can be written as (A), where A is a valid string.
//!  
//!
//! Example 1:
//!
//! Input: s = "lee(t(c)o)de)"
//! Output: "lee(t(c)o)de"
//! Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
//! Example 2:
//!
//! Input: s = "a)b(c)d"
//! Output: "ab(c)d"
//! Example 3:
//!
//! Input: s = "))(("
//! Output: ""
//! Explanation: An empty string is also valid.
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 10^5
//! s[i] is either '(' , ')', or lowercase English letter.

// Remarks:
// - A closing parenthesis without a previous open one is surely to be discarded (you can remove parenthesis, not adding it)
// - All the opening parenthesis should be closed as well: a counter might do this job
use super::*;
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut flag_open: u32 = 0;
        let mut flag_closed: u32 = 0;
        let n = s.len();

        let mut out: Vec<Option<char>> = s.chars().map(Some).collect::<Vec<_>>();

        for i in 0..n {
            let f = out[i];
            let b = out[n - 1 - i];

            //forward iterate
            if f == Some(')') {
                if flag_open == 0 {
                    out[i] = None
                }
                flag_open = flag_open.saturating_sub(1);
            } else if f == Some('(') {
                flag_open += 1;
            }

            //baxckward iterate
            if b == Some('(') {
                if flag_closed == 0 {
                    out[n - 1 - i] = None
                }
                flag_closed = flag_closed.saturating_sub(1);
            } else if b == Some(')') {
                flag_closed += 1;
            }
        }

        out.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1249() {
        let s = "lee(t(c)o)de)";
        let out = Solution::min_remove_to_make_valid(s.to_string());
        assert!(&out == "lee(t(c)o)de" || &out == "lee(t(c)ode)");

        let s = "a)b(c)d";
        let out = Solution::min_remove_to_make_valid(s.to_string());
        assert!(&out == "ab(c)d");

        let s = "))((";
        let out = Solution::min_remove_to_make_valid(s.to_string());
        assert!(&out.is_empty());
    }
}
