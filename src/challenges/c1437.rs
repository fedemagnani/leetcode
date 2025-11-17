//! Given an binary array nums and an integer k, return true if all 1's are at least k places away from each other, otherwise return false.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: nums = [1,0,0,0,1,0,0,1], k = 2
//! Output: true
//! Explanation: Each of the 1s are at least 2 places away from each other.
//! Example 2:
//!
//!
//! Input: nums = [1,0,0,1,0,1], k = 2
//! Output: false
//! Explanation: The second 1 and third 1 are only one apart from each other.
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length <= 105
//! 0 <= k <= nums.length
//! nums[i] is 0 or 1

use super::*;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = -1000000000;
        for i in nums.iter() {
            let i = *i;
            if nums[i as usize] == 1 {
                if (i) - last - 1 < k {
                    return false;
                }
                last = i;
            }
        }
        true
    }
}
