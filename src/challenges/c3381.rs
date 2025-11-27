//! You are given an array of integers nums and an integer k.
//!
//! Return the maximum sum of a subarray of nums, such that the size of the subarray is divisible by k.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [1,2], k = 1
//!
//! Output: 3
//!
//! Explanation:
//!
//! The subarray [1, 2] with sum 3 has length equal to 2 which is divisible by 1.
//!
//! Example 2:
//!
//! Input: nums = [-1,-2,-3,-4,-5], k = 4
//!
//! Output: -10
//!
//! Explanation:
//!
//! The maximum sum subarray is [-1, -2, -3, -4] which has length equal to 4 which is divisible by 4.
//!
//! Example 3:
//!
//! Input: nums = [-5,1,2,-3,4], k = 2
//!
//! Output: 4
//!
//! Explanation:
//!
//! The maximum sum subarray is [1, 2, -3, 4] which has length equal to 4 which is divisible by 2.
//!
//!  
//!
//! Constraints:
//!
//! 1 <= k <= nums.length <= 2 * 105
//! -109 <= nums[i] <= 109

use super::*;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut prefix_sum: i64 = 0;
        let mut max_sum: i64 = i64::MIN;
        let k = k as usize;
        let mut k_sum: Vec<i64> = vec![i64::MAX / 2; k];
        k_sum[k - 1] = 0;
        for (i, num) in nums.into_iter().enumerate() {
            prefix_sum += num as i64;
            let idx = i % k;
            max_sum = max_sum.max(prefix_sum - k_sum[idx]);
            k_sum[idx] = k_sum[idx].min(prefix_sum);
        }
        max_sum
    }
}
