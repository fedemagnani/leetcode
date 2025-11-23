//! Given an integer array nums, return the maximum possible sum of elements of the array such that it is divisible by three.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [3,6,5,1,8]
//! Output: 18
//! Explanation: Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).
//! Example 2:
//!
//! Input: nums = [4]
//! Output: 0
//! Explanation: Since 4 is not divisible by 3, do not pick any number.
//! Example 3:
//!
//! Input: nums = [1,2,3,4,4]
//! Output: 12
//! Explanation: Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length <= 4 * 104
//! 1 <= nums[i] <= 104

use super::*;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut sum = 0;

        let mut m1a = i32::MAX;
        let mut m1b = i32::MAX;
        let mut m2a = i32::MAX;
        let mut m2b = i32::MAX;

        for x in nums {
            sum += x;
            let r = x % 3;

            if r == 1 {
                if x < m1a {
                    m1b = m1a;
                    m1a = x;
                } else if x < m1b {
                    m1b = x;
                }
            } else if r == 2 {
                if x < m2a {
                    m2b = m2a;
                    m2a = x;
                } else if x < m2b {
                    m2b = x;
                }
            }
        }

        let rem = sum % 3;
        if rem == 0 {
            return sum;
        }

        let remove = if rem == 1 {
            let opt1 = m1a;
            let opt2 = if m2b < i32::MAX { m2a + m2b } else { i32::MAX };
            opt1.min(opt2)
        } else {
            let opt1 = m2a;
            let opt2 = if m1b < i32::MAX { m1a + m1b } else { i32::MAX };
            opt1.min(opt2)
        };

        if remove == i32::MAX { 0 } else { sum - remove }
    }
}
