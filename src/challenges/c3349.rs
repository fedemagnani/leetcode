//! Given an array nums of n integers and an integer k, determine whether there exist two adjacent subarrays of length k such that both subarrays are strictly increasing. Specifically, check if there are two subarrays starting at indices a and b (a < b), where:
//!
//! Both subarrays nums[a..a + k - 1] and nums[b..b + k - 1] are strictly increasing.
//! The subarrays must be adjacent, meaning b = a + k.
//! Return true if it is possible to find two such subarrays, and false otherwise.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [2,5,7,8,9,2,3,4,3,1], k = 3
//!
//! Output: true
//!
//! Explanation:
//!
//! The subarray starting at index 2 is [7, 8, 9], which is strictly increasing.
//! The subarray starting at index 5 is [2, 3, 4], which is also strictly increasing.
//! These two subarrays are adjacent, so the result is true.
//! Example 2:
//!
//! Input: nums = [1,2,3,4,4,4,4,5,6,7], k = 5
//!
//! Output: false
//!
//!  
//!
//! Constraints:
//!
//! 2 <= nums.length <= 100
//! 1 < 2 * k <= nums.length
//! -1000 <= nums[i] <= 1000

use super::*;

impl Solution {
    fn scan_forward(nums: &[i32], i: usize, k: usize) -> bool {
        let mut increasing = true;

        for j in i..i + k - 1 {
            if nums[j] >= nums[j + 1] {
                increasing = false;
                break;
            }
        }
        if !increasing {
            return false;
        }
        for j in i + k..i + 2 * k - 1 {
            if nums[j] >= nums[j + 1] {
                increasing = false;
                break;
            }
        }
        if increasing {
            return true;
        }
        false
    }

    fn scan_bacward(nums: &[i32], i: usize, k: usize) -> bool {
        let mut increasing = true;
        println!("{:?}", &nums[i - k..i]);

        for j in i - k..i - 1 {
            if nums[j] >= nums[j + 1] {
                increasing = false;
                break;
            }
        }

        if !increasing {
            return false;
        }
        for j in i - 2 * k..i - k - 1 {
            if nums[j] >= nums[j + 1] {
                increasing = false;
                break;
            }
        }
        if increasing {
            return true;
        }
        false
    }
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let k = k as usize;
        // scan forward and backward
        for i in 0..n - 1 {
            let j = n - i;
            if i + 2 * k > n || j < 2 * k || i > j {
                break;
            }
            if Self::scan_forward(&nums, i, k) {
                return true;
            }
            if Self::scan_bacward(&nums, j, k) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test3349() {
        // let nums = [2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        // let k = 3;
        // let out = Solution::has_increasing_subarrays(nums.to_vec(), k);
        // debug_assert!(out);

        // let nums = [1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        // let k = 5;
        // let out = Solution::has_increasing_subarrays(nums.to_vec(), k);
        // debug_assert!(!out);

        let nums = [-15, 19];
        let k = 1;
        let out = Solution::has_increasing_subarrays(nums.to_vec(), k);
        debug_assert!(out);
    }
}
