//! You are given a binary string s.
//!
//! Return the number of substrings with dominant ones.
//!
//! A string has dominant ones if the number of ones in the string is greater than or equal to the square of the number of zeros in the string.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "00011"
//!
//! Output: 5
//!
//! Explanation:
//!
//! The substrings with dominant ones are shown in the table below.
//!
//! i   j   s[i..j] Number of Zeros Number of Ones
//! 3   3   1   0   1
//! 4   4   1   0   1
//! 2   3   01  1   1
//! 3   4   11  0   2
//! 2   4   011 1   2
//! Example 2:
//!
//! Input: s = "101101"
//!
//! Output: 16
//!
//! Explanation:
//!
//! The substrings with non-dominant ones are shown in the table below.
//!
//! Since there are 21 substrings total and 5 of them have non-dominant ones, it follows that there are 16 substrings with dominant ones.
//!
//! i    j  s[i..j] Number of Zeros Number of Ones
//! 1   1   0   1   0
//! 4   4   0   1   0
//! 1   4   0110    2   2
//! 0   4   10110   2   3
//! 1   5   01101   2   3
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 4 * 104
//! s consists only of characters '0' and '1'.

use super::*;
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut pre = vec![-1; n + 1];
        for i in 0..n {
            if i == 0 || chars[i - 1] == '0' {
                pre[i + 1] = i as i32;
            } else {
                pre[i + 1] = pre[i];
            }
        }

        let mut res = 0i32;
        for i in 1..=n {
            let mut cnt0 = if chars[i - 1] == '0' { 1 } else { 0 };
            let mut j = i as i32;
            while j > 0 && (cnt0 * cnt0) as usize <= n {
                let cnt1 = (i as i32 - pre[j as usize]) - cnt0;
                if cnt0 * cnt0 <= cnt1 {
                    res += std::cmp::min(j - pre[j as usize], cnt1 - cnt0 * cnt0 + 1);
                }
                j = pre[j as usize];
                cnt0 += 1;
            }
        }
        res
    }
}
