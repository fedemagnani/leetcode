//! An integer x is numerically balanced if for every digit d in the number x, there are exactly d occurrences of that digit in x.
//!
//! Given an integer n, return the smallest numerically balanced number strictly greater than n.
//!
//!  
//!
//! Example 1:
//!
//! Input: n = 1
//! Output: 22
//! Explanation:
//! 22 is numerically balanced since:
//! - The digit 2 occurs 2 times.
//!   It is also the smallest numerically balanced number strictly greater than 1.
//!   Example 2:
//!
//! Input: n = 1000
//! Output: 1333
//! Explanation:
//! 1333 is numerically balanced since:
//! - The digit 1 occurs 1 time.
//! - The digit 3 occurs 3 times.
//!   It is also the smallest numerically balanced number strictly greater than 1000.
//!   Note that 1022 cannot be the answer because 0 appeared more than 0 times.
//!   Example 3:
//!
//! Input: n = 3000
//! Output: 3133
//! Explanation:
//! 3133 is numerically balanced since:
//! - The digit 1 occurs 1 time.
//! - The digit 3 occurs 3 times.
//!   It is also the smallest numerically balanced number strictly greater than 3000.
//!  
//!
//! Constraints:
//!
//! 0 <= n <= 106

// ! All the candidates are expected to be permutations of 1, 22, 333, ..., 999999999
// ! The sequence of candidates is 1, 22, 122, 212, 221, 333, 1333, 3133,
use super::*;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut next = n + 1;

        loop {
            if Self::is_beautifull(next) {
                return next;
            }
            next += 1;
        }
    }
    fn is_beautifull(mut num: i32) -> bool {
        let mut freqs = [0_i32; 10];

        while num > 0 {
            freqs[(num % 10) as usize] += 1;
            num /= 10;
        }

        for (i, f) in freqs.into_iter().enumerate() {
            if f != 0 && f != i as i32 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2048() {
        let n = 1;
        let exp = 22;
        let out = Solution::next_beautiful_number(n);
        debug_assert_eq!(exp, out);

        let n = 1000;
        let exp = 1333;
        let out = Solution::next_beautiful_number(n);
        debug_assert_eq!(exp, out);

        let n = 3000;
        let exp = 3133;
        let out = Solution::next_beautiful_number(n);
        debug_assert_eq!(exp, out);
    }
}
