//! Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
//!
//! Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
//!
//!  
//!
//! Example 1:
//!
//! Input: x = 123
//! Output: 321
//! Example 2:
//!
//! Input: x = -123
//! Output: -321
//! Example 3:
//!
//! Input: x = 120
//! Output: 21
//!  
//!
//! Constraints:
//!
//! -231 <= x <= 231 - 1

use super::*;
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut out: i32 = 0;
        let negative = x < 0;
        if negative {
            x *= -1;
        }

        while x != 0 {
            let Some(res) = out.checked_mul(10) else {
                return 0;
            };
            out = res;
            let Some(res) = out.checked_add(x % 10) else {
                return 0;
            };
            out = res;
            x /= 10;
        }

        if negative {
            out *= -1;
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test7() {
        let x = 123;
        let out = Solution::reverse(x);
        debug_assert_eq!(out, 321);

        let x = -123;
        let out = Solution::reverse(x);
        debug_assert_eq!(out, -321);

        let x = -120;
        let out = Solution::reverse(x);
        debug_assert_eq!(out, -21);

        let x = 1534236469;
        let out = Solution::reverse(x);
        debug_assert_eq!(out, 0);
    }
}
