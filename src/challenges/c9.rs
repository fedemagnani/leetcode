//! Given an integer x, return true if x is a palindrome, and false otherwise.
//!
//!  
//!
//! Example 1:
//!
//! Input: x = 121
//! Output: true
//! Explanation: 121 reads as 121 from left to right and from right to left.
//! Example 2:
//!
//! Input: x = -121
//! Output: false
//! Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//! Example 3:
//!
//! Input: x = 10
//! Output: false
//! Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//!  
//!
//! Constraints:
//!
//! -231 <= x <= 231 - 1

use super::*;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut num = x;
        let mut rev = 0;

        while num > 0 {
            rev *= 10;
            let digit = num % 10;
            rev += digit;
            num /= 10;
        }

        rev == x
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn reverse_number() {
        let mut num = 18483;

        let mut rev = 0;

        while num > 0 {
            rev *= 10; //0, 30, 380...
            let digit = num % 10; //3, 8, 4, ...
            rev += digit; //3, 38, 384, ...
            num /= 10; //1848, 184, 18, ...
        }

        println!("{rev}")
    }
}
