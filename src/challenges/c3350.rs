//! Given an array nums of n integers, your task is to find the maximum value of k for which there exist two adjacent subarrays of length k each, such that both subarrays are strictly increasing. Specifically, check if there are two subarrays of length k starting at indices a and b (a < b), where:
//!
//! Both subarrays nums[a..a + k - 1] and nums[b..b + k - 1] are strictly increasing.
//! The subarrays must be adjacent, meaning b = a + k.
//! Return the maximum possible value of k.
//!
//! A subarray is a contiguous non-empty sequence of elements within an array.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [2,5,7,8,9,2,3,4,3,1]
//!
//! Output: 3
//!
//! Explanation:
//!
//! The subarray starting at index 2 is [7, 8, 9], which is strictly increasing.
//! The subarray starting at index 5 is [2, 3, 4], which is also strictly increasing.
//! These two subarrays are adjacent, and 3 is the maximum possible value of k for which two such adjacent strictly increasing subarrays exist.
//! Example 2:
//!
//! Input: nums = [1,2,3,4,4,4,4,5,6,7]
//!
//! Output: 2
//!
//! Explanation:
//!
//! The subarray starting at index 0 is [1, 2], which is strictly increasing.
//! The subarray starting at index 2 is [3, 4], which is also strictly increasing.
//! These two subarrays are adjacent, and 2 is the maximum possible value of k for which two such adjacent strictly increasing subarrays exist.
//!  
//!
//! Constraints:
//!
//! 2 <= nums.length <= 2 * 105
//! -109 <= nums[i] <= 109

use super::*;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sequences = Vec::with_capacity(n);

        let mut i = 0;

        let mut highest = 0;

        while i < n - 1 {
            i += 1;

            let mut v: usize = 1;
            while i < n && nums[i - 1] < nums[i] {
                i += 1;
                v += 1;
            }

            if v > highest {
                highest = v;
            }

            sequences.push(v);
        }

        let mut out = 1;

        for seqs in sequences.windows(2) {
            let len0 = seqs[0];
            let len1 = seqs[1];
            let c = len0.min(len1);
            if c > out {
                out = c
            }
        }

        let h = highest / 2;
        if h > out {
            return h as i32;
        }

        out as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test3350() {
        let nums = [2, 5, 7, 8, 9, 2, 3, 4, 3, 1].to_vec();
        let exp = 3;
        let out = Solution::max_increasing_subarrays(nums);
        debug_assert_eq!(exp, out);

        let nums = [1, 2, 3, 4, 4, 4, 4, 5, 6, 7].to_vec();
        let exp = 2;
        let out = Solution::max_increasing_subarrays(nums);
        debug_assert_eq!(exp, out);

        let nums = [-15, 19].to_vec();
        let exp = 1;
        let out = Solution::max_increasing_subarrays(nums);
        debug_assert_eq!(exp, out);

        let nums = [5, 8, -2, -1].to_vec();
        let exp = 2;
        let out = Solution::max_increasing_subarrays(nums);
        debug_assert_eq!(exp, out);
    }
}
