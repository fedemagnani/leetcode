//! Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
//!
//! Return the sum of the three integers.
//!
//! You may assume that each input would have exactly one solution.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [-1,2,1,-4], target = 1
//! Output: 2
//! Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
//! Example 2:
//!
//! Input: nums = [0,0,0], target = 1
//! Output: 0
//! Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
//!  
//!
//! Constraints:
//!
//! 3 <= nums.length <= 500
//! -1000 <= nums[i] <= 1000
//! -104 <= target <= 104

use super::*;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut inf_diff = i32::MAX;
        let mut positive = true;

        let n = nums.len();
        for i in 0..n - 2 {
            let mut low = i + 1;
            let mut high = n - 1;

            let inner_t = target - nums[i];

            while low < high {
                let t = nums[low] + nums[high];
                let diff = inner_t - t;

                if diff == 0 {
                    return target;
                // if diff < 0, it means that 3 summ is too high, so we reduce upper bound
                } else if diff < 0 {
                    high -= 1;

                    if -diff < inf_diff {
                        inf_diff = -diff;
                        positive = false;
                    }
                } else {
                    low += 1;
                    if diff < inf_diff {
                        inf_diff = diff;
                        positive = true;
                    }
                }
            }
        }
        if positive {
            return target - inf_diff;
        }
        target + inf_diff
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test16() {
        let nums = [-1, 2, 1, -4];
        let target = 1;
        let exp = 2;
        let out = Solution::three_sum_closest(nums.to_vec(), target);
        debug_assert_eq!(exp, out);

        let nums = [0, 0, 0];
        let target = 1;
        let exp = 0;
        let out = Solution::three_sum_closest(nums.to_vec(), target);
        debug_assert_eq!(exp, out);

        let nums = [10, 20, 30, 40, 50, 60, 70, 80, 90];
        let target = 1;
        let exp = 60;
        let out = Solution::three_sum_closest(nums.to_vec(), target);
        debug_assert_eq!(exp, out);
    }
}
