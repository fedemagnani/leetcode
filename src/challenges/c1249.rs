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

// ( [40]
// ) [41]
// we set u8::MAX as marker for removable index
use super::*;
impl Solution {
    pub fn min_remove_to_make_valid(mut s: String) -> String {
        let mut flag_open: u32 = 0;
        let mut flag_closed: u32 = 0;
        let n = s.len();

        let raw = unsafe { s.as_bytes_mut() };

        for i in 0..n {
            let f = raw[i];
            let b = raw[n - 1 - i];

            //forward iterate

            if 41.eq(&f) {
                if flag_open == 0 {
                    raw[i] = u8::MAX;
                }
                flag_open = flag_open.saturating_sub(1);
            } else if 40.eq(&f) {
                flag_open += 1;
            }

            //baxckward iterate
            if 40.eq(&b) {
                if flag_closed == 0 {
                    raw[n - 1 - i] = u8::MAX;
                }
                flag_closed = flag_closed.saturating_sub(1);
            } else if 41.eq(&b) {
                flag_closed += 1;
            }
        }

        let raw = raw.iter().filter(|x| !u8::MAX.eq(x));

        let mut s = String::with_capacity(raw.size_hint().0);
        unsafe {
            let vec = s.as_mut_vec(); // get underlying Vec<u8>
            vec.extend(raw.cloned()); // extend in-place
        }

        s
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

    #[test]
    fn byte() {
        println!("( {:?}", "(".as_bytes());
        println!(") {:?}", ")".as_bytes());
        let s = String::from_utf8_lossy(&[u8::MAX]);
        println!("{s}");
    }
}
