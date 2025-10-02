//! Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.
//!
//! The algorithm for myAtoi(string s) is as follows:
//!
//! Whitespace: Ignore any leading whitespace (" ").
//! Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity if neither present.
//! Conversion: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
//! Rounding: If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then round the integer to remain in the range. Specifically, integers less than -231 should be rounded to -231, and integers greater than 231 - 1 should be rounded to 231 - 1.
//! Return the integer as the final result.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "42"
//!
//! Output: 42
//!
//! Explanation:
//!
//! The underlined characters are what is read in and the caret is the current reader position.
//! Step 1: "42" (no characters read because there is no leading whitespace)
//!          ^
//! Step 2: "42" (no characters read because there is neither a '-' nor '+')
//!          ^
//! Step 3: "42" ("42" is read in)
//!            ^
//! Example 2:
//!
//! Input: s = " -042"
//!
//! Output: -42
//!
//! Explanation:
//!
//! Step 1: "   -042" (leading whitespace is read and ignored)
//!             ^
//! Step 2: "   -042" ('-' is read, so the result should be negative)
//!              ^
//! Step 3: "   -042" ("042" is read in, leading zeros ignored in the result)
//!                ^
//! Example 3:
//!
//! Input: s = "1337c0d3"
//!
//! Output: 1337
//!
//! Explanation:
//!
//! Step 1: "1337c0d3" (no characters read because there is no leading whitespace)
//!          ^
//! Step 2: "1337c0d3" (no characters read because there is neither a '-' nor '+')
//!          ^
//! Step 3: "1337c0d3" ("1337" is read in; reading stops because the next character is a non-digit)
//!              ^
//! Example 4:
//!
//! Input: s = "0-1"
//!
//! Output: 0
//!
//! Explanation:
//!
//! Step 1: "0-1" (no characters read because there is no leading whitespace)
//!          ^
//! Step 2: "0-1" (no characters read because there is neither a '-' nor '+')
//!          ^
//! Step 3: "0-1" ("0" is read in; reading stops because the next character is a non-digit)
//!           ^
//! Example 5:
//!
//! Input: s = "words and 987"
//!
//! Output: 0
//!
//! Explanation:
//!
//! Reading stops at the first non-digit character 'w'.
//!
//!  
//!
//! Constraints:
//!
//! 0 <= s.length <= 200
//! s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.

// In ASCII, [0,...,9] are bytes [48,...,57]
// As a result, an easy casting method is byte-48
//

use super::*;

use std::ops::Sub;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let l = s.len();
        if l == 0 {
            return 0;
        }
        let mut digits = Vec::<&u8>::with_capacity(l);
        let mut chars = s.as_bytes().iter();

        // trim initial and ending spaces
        let first_digit = loop {
            let n = chars.next();
            let Some(n) = n else { return 0 };
            if n == &32 {
                continue;
            }
            break n;
        };

        let is_negative = first_digit == &45;

        let is_number = (&48..=&57).contains(&first_digit);

        if first_digit != &43 && !is_number && !is_negative {
            return 0;
        }
        let is_zero = first_digit == &48;

        let mut no_nums = true;
        if is_number && !is_zero {
            digits.push(first_digit);
            no_nums = false;
        }
        for c in chars {
            let is_number = (&48..=&57).contains(&c);
            let is_zero = c == &48;

            if is_zero && no_nums {
                continue;
            }

            if !is_number {
                break;
            }
            digits.push(c);
            no_nums = false;
        }

        let mut out: i32 = 0;
        let d_len = digits.len();
        let mut overflow = false;
        for (i, d) in digits.iter().enumerate() {
            let Some(dec) = 10_i32.checked_pow((d_len - i - 1) as u32) else {
                overflow = true;
                break;
            };

            let Some(addend) = dec.checked_mul(d.sub(48) as i32) else {
                overflow = true;
                break;
            };

            let Some(res) = out.checked_add(addend) else {
                overflow = true;

                break;
            };

            out = res;
        }

        if is_negative {
            if overflow {
                return i32::MIN;
            }
            return -out;
        }

        if overflow {
            out = i32::MAX;
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test8() {
        // let s = "42";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, 42);

        // let s = " -042";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, -42);

        // let s = "1337c0d3";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, 1337);

        // let s = "0-1";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, 0);

        // let s = "words and 987";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, 0);

        // let s = "-91283472332";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, -2147483648);

        // let s = "  0000000000012345678";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, 12345678);

        // let s = "-6147483648";
        // let out = Solution::my_atoi(s.to_string());
        // debug_assert_eq!(out, -2147483648);

        let s = "20000000000000000000";
        let out = Solution::my_atoi(s.to_string());
        debug_assert_eq!(out, 2147483647);
    }

    #[test]
    fn t() {
        debug_assert_eq!(48, "0".as_bytes()[0]);
        debug_assert_eq!(57, "9".as_bytes()[0]);
        debug_assert_eq!(45, "-".as_bytes()[0]);
        debug_assert_eq!(43, "+".as_bytes()[0]);
        debug_assert_eq!(32, " ".as_bytes()[0]);
    }
}
