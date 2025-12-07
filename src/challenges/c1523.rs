//! Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).
//!
//!  
//!
//! Example 1:
//!
//! Input: low = 3, high = 7
//! Output: 3
//! Explanation: The odd numbers between 3 and 7 are [3,5,7].
//! Example 2:
//!
//! Input: low = 8, high = 10
//! Output: 1
//! Explanation: The odd numbers between 8 and 10 are [9].
//!  
//!
//! Constraints:
//!
//! 0 <= low <= high <= 10^9

use super::*;

impl Solution {
    /// Recall that x>>1 returns the number of odd numbers inside [0, x)
    /// As a result, (high + 1) >> 1 counts the number of odds number in interval [0, high]
    /// However, since we need to restrict the search just for interval [low,high], we also need to subtract all the odd numbers
    /// in range [0,low)
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high + 1) >> 1) - (low >> 1)
    }
}
