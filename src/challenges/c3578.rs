//! You are given an integer array nums and an integer k. Your task is to partition nums into one or more non-empty contiguous segments such that in each segment, the difference between its maximum and minimum elements is at most k.
//!
//! Return the total number of ways to partition nums under this condition.
//!
//! Since the answer may be too large, return it modulo 109 + 7.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [9,4,1,3,7], k = 4
//!
//! Output: 6
//!
//! Explanation:
//!
//! There are 6 valid partitions where the difference between the maximum and minimum elements in each segment is at most k = 4:
//!
//! [[9], [4], [1], [3], [7]]
//! [[9], [4], [1], [3, 7]]
//! [[9], [4], [1, 3], [7]]
//! [[9], [4, 1], [3], [7]]
//! [[9], [4, 1], [3, 7]]
//! [[9], [4, 1, 3], [7]]
//! Example 2:
//!
//! Input: nums = [3,3,4], k = 0
//!
//! Output: 2
//!
//! Explanation:
//!
//! There are 2 valid partitions that satisfy the given conditions:
//!
//! [[3], [3], [4]]
//! [[3, 3], [4]]
//!  
//!
//! Constraints:
//!
//! 2 <= nums.length <= 5 * 104
//! 1 <= nums[i] <= 109
//! 0 <= k <= 109

use super::*;

use std::collections::BTreeMap;

impl Solution {
    pub fn count_partitions_2(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mod_val = 1_000_000_007i64;
        let mut dp = vec![0i64; n + 1];
        let mut prefix = vec![0i64; n + 1];
        let mut cnt = BTreeMap::new();

        dp[0] = 1;
        prefix[0] = 1;
        let mut j = 0;
        for i in 0..n {
            *cnt.entry(nums[i]).or_insert(0) += 1;
            // adjust window
            while j <= i && *cnt.keys().last().unwrap() - *cnt.keys().next().unwrap() > k {
                *cnt.get_mut(&nums[j]).unwrap() -= 1;
                if cnt[&nums[j]] == 0 {
                    cnt.remove(&nums[j]);
                }
                j += 1;
            }

            if j > 0 {
                dp[i + 1] = (prefix[i] - prefix[j - 1] + mod_val) % mod_val;
            } else {
                dp[i + 1] = prefix[i] % mod_val;
            }
            prefix[i + 1] = (prefix[i] + dp[i + 1]) % mod_val;
        }

        dp[n] as i32
    }
}
