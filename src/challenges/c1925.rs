//! A square triple (a,b,c) is a triple where a, b, and c are integers and a2 + b2 = c2.
//!
//! Given an integer n, return the number of square triples such that 1 <= a, b, c <= n.
//!
//!  
//!
//! Example 1:
//!
//! Input: n = 5
//! Output: 2
//! Explanation: The square triples are (3,4,5) and (4,3,5).
//! Example 2:
//!
//! Input: n = 10
//! Output: 4
//! Explanation: The square triples are (3,4,5), (4,3,5), (6,8,10), and (8,6,10).

//! Constraints:
//!
//! 1 <= n <= 250

use super::*;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut res = 0;
        // enumerate a and b
        for a in 1..=n {
            for b in 1..=n {
                // determine if it meets the requirements
                let c = ((a * a + b * b) as f64).sqrt().floor() as i32;
                if c <= n && c * c == a * a + b * b {
                    res += 1;
                }
            }
        }
        res
    }
}
