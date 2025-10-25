//! Hercy wants to save money for his first car. He puts money in the Leetcode bank every day.
//!
//! He starts by putting in $1 on Monday, the first day. Every day from Tuesday to Sunday, he will put in $1 more than the day before. On every subsequent Monday, he will put in $1 more than the previous Monday.
//!
//! Given n, return the total amount of money he will have in the Leetcode bank at the end of the nth day.
//!
//!  
//!
//! Example 1:
//!
//! Input: n = 4
//! Output: 10
//! Explanation: After the 4th day, the total is 1 + 2 + 3 + 4 = 10.
//! Example 2:
//!
//! Input: n = 10
//! Output: 37
//! Explanation: After the 10th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4) = 37. Notice that on the 2nd Monday, Hercy only puts in $2.
//! Example 3:
//!
//! Input: n = 20
//! Output: 96
//! Explanation: After the 20th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96.
//!  
//!
//! Constraints:
//!
//! 1 <= n <= 1000
use super::*;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let q = n / 7;
        let r = n % 7;
        28 * q + 7 * q * (q - 1) / 2 + (2 * q + r + 1) * r / 2
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1716() {
        let n = 4;
        let exp = 10;
        let out = Solution::total_money(n);
        debug_assert_eq!(exp, out);

        let n = 10;
        let exp = 37;
        let out = Solution::total_money(n);
        debug_assert_eq!(exp, out);

        let n = 20;
        let exp = 96;
        let out = Solution::total_money(n);
        debug_assert_eq!(exp, out);

        let n = 4;
        let exp = 10;
        let out = Solution::total_money(n);
        debug_assert_eq!(exp, out);
    }
}
