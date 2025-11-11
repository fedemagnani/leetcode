//!You are given an array of binary strings strs and two integers m and n.
//!
//!Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the subset.
//!
//!A set x is a subset of a set y if all elements of x are also elements of y.
//!
//!
//!
//!Example 1:
//!
//!Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
//!Output: 4
//!Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
//!Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
//!{"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
//!Example 2:
//!
//!Input: strs = ["10","0","1"], m = 1, n = 1
//!Output: 2
//!Explanation: The largest subset is {"0", "1"}, so the answer is 2.
//!
//!
//!Constraints:
//!
//!1 <= strs.length <= 600
//!1 <= strs[i].length <= 100
//!strs[i] consists only of digits '0' and '1'.
//!1 <= m, n <= 100

use super::*;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
        for s in strs {
            let (zeros, ones) = Self::count_zero_one(&s);
            for i in (zeros..=m).rev() {
                for j in (ones..=n).rev() {
                    let i_usize = i as usize;
                    let j_usize = j as usize;
                    dp[i_usize][j_usize] =
                        dp[i_usize][j_usize].max(dp[(i - zeros) as usize][(j - ones) as usize] + 1);
                }
            }
        }
        dp[m as usize][n as usize]
    }

    fn count_zero_one(s: &str) -> (i32, i32) {
        let zeros = s.chars().filter(|&c| c == '0').count() as i32;
        let ones = s.len() as i32 - zeros;
        (zeros, ones)
    }
}
