//! You are given a 0-indexed integer array nums and an integer value.
//!
//! In one operation, you can add or subtract value from any element of nums.
//!
//! For example, if nums = [1,2,3] and value = 2, you can choose to subtract value from nums[0] to make nums = [-1,2,3].
//! The MEX (minimum excluded) of an array is the smallest missing non-negative integer in it.
//!
//! For example, the MEX of [-1,2,3] is 0 while the MEX of [1,0,3] is 2.
//! Return the maximum MEX of nums after applying the mentioned operation any number of times.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [1,-10,7,13,6,8], value = 5
//! Output: 4
//! Explanation: One can achieve this result by applying the following operations:
//! - Add value to nums[1] twice to make nums = [1,0,7,13,6,8]
//! - Subtract value from nums[2] once to make nums = [1,0,2,13,6,8]
//! - Subtract value from nums[3] twice to make nums = [1,0,2,3,6,8]
//!   The MEX of nums is 4. It can be shown that 4 is the maximum MEX we can achieve.
//!   Example 2:
//!
//! Input: nums = [1,-10,7,13,6,8], value = 7
//! Output: 2
//! Explanation: One can achieve this result by applying the following operation:
//! - subtract value from nums[2] once to make nums = [1,-10,0,13,6,8]
//!   The MEX of nums is 2. It can be shown that 2 is the maximum MEX we can achieve.
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length, value <= 105
//! -109 <= nums[i] <= 109

use super::*;
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut mp = vec![0; value as usize];
        nums.iter().for_each(|&x| {
            let pos_mod = (x % value + value) % value;
            mp[pos_mod as usize] += 1
        });
        let mut mex = 0;
        while mp[(mex % value) as usize] > 0 {
            mp[(mex % value) as usize] -= 1;
            mex += 1;
        }
        mex
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test2598() {
        let nums = [1, -10, 7, 13, 6, 8];
        let value = 5;
        let exp = 4;
        let out = Solution::find_smallest_integer(nums.to_vec(), value);
        debug_assert_eq!(exp, out);

        let nums = [1, -10, 7, 13, 6, 8];
        let value = 7;
        let exp = 2;
        let out = Solution::find_smallest_integer(nums.to_vec(), value);
        debug_assert_eq!(exp, out);

        let nums = [3, 0, 3, 2, 4, 2, 1, 1, 0, 4];
        let value = 5;
        let exp = 10;
        let out = Solution::find_smallest_integer(nums.to_vec(), value);
        debug_assert_eq!(exp, out);

        let nums = [-6, -3, 4];
        let value = 2;
        let exp = 3;
        let out = Solution::find_smallest_integer(nums.to_vec(), value);
        debug_assert_eq!(exp, out);

        let nums = [-4, 6, 5];
        let value = 1;
        let exp = 3;
        let out = Solution::find_smallest_integer(nums.to_vec(), value);
        debug_assert_eq!(exp, out);
    }
}
