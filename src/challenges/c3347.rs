//! You are given an integer array nums and two integers k and numOperations.
//!
//! You must perform an operation numOperations times on nums, where in each operation you:
//!
//! Select an index i that was not selected in any previous operations.
//! Add an integer in the range [-k, k] to nums[i].
//! Return the maximum possible frequency of any element in nums after performing the operations.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [1,4,5], k = 1, numOperations = 2
//!
//! Output: 2
//!
//! Explanation:
//!
//! We can achieve a maximum frequency of two by:
//!
//! Adding 0 to nums[1], after which nums becomes [1, 4, 5].
//! Adding -1 to nums[2], after which nums becomes [1, 4, 4].
//! Example 2:
//!
//! Input: nums = [5,11,20,20], k = 5, numOperations = 1
//!
//! Output: 2
//!
//! Explanation:
//!
//! We can achieve a maximum frequency of two by:
//!
//! Adding 0 to nums[1].
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length <= 105
//! 1 <= nums[i] <= 109
//! 0 <= k <= 109
//! 0 <= numOperations <= nums.length

use super::*;
use std::cmp;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_frequency_2(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut ans = 0;
        let mut num_count = HashMap::new();
        let mut modes = HashSet::new();

        let mut add_mode = |value: i32| {
            modes.insert(value);
            if value - k >= nums[0] {
                modes.insert(value - k);
            }
            if value + k <= nums[nums.len() - 1] {
                modes.insert(value + k);
            }
        };

        let mut last_num_index = 0;
        for i in 0..nums.len() {
            if nums[i] != nums[last_num_index] {
                num_count.insert(nums[last_num_index], i - last_num_index);
                ans = cmp::max(ans, (i - last_num_index) as i32);
                add_mode(nums[last_num_index]);
                last_num_index = i;
            }
        }

        num_count.insert(nums[last_num_index], nums.len() - last_num_index);
        ans = cmp::max(ans, (nums.len() - last_num_index) as i32);
        add_mode(nums[last_num_index]);

        let left_bound = |value: i32| -> usize {
            let mut left = 0;
            let mut right = nums.len() - 1;
            while left < right {
                let mid = (left + right) / 2;
                if nums[mid] < value {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        };

        let right_bound = |value: i32| -> usize {
            let mut left = 0;
            let mut right = nums.len() - 1;
            while left < right {
                let mid = (left + right).div_ceil(2);
                if nums[mid] > value {
                    right = mid - 1;
                } else {
                    left = mid;
                }
            }
            left
        };

        let mut sorted_modes: Vec<i32> = modes.into_iter().collect();
        sorted_modes.sort();

        for mode in sorted_modes {
            let l = left_bound(mode - k);
            let r = right_bound(mode + k);
            let temp_ans = if let Some(&count) = num_count.get(&mode) {
                cmp::min(r - l + 1, count + num_operations as usize)
            } else {
                cmp::min(r - l + 1, num_operations as usize)
            };
            ans = cmp::max(ans, temp_ans as i32);
        }

        ans
    }
}

#[cfg(test)]
mod tes {
    use super::*;
    #[test]
    fn test3347() {
        let nums = [1, 4, 5];
        let k = 1;
        let num_operations = 2;
        let exp = 2;
        let out = Solution::max_frequency_2(nums.to_vec(), k, num_operations);
        debug_assert_eq!(exp, out);

        let nums = [5, 11, 20, 20];
        let k = 5;
        let num_operations = 1;
        let exp = 2;
        let out = Solution::max_frequency_2(nums.to_vec(), k, num_operations);
        debug_assert_eq!(exp, out);
    }
}
