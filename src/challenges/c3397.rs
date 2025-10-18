//! You are given an integer array nums and an integer k.
//!
//! You are allowed to perform the following operation on each element of the array at most once:
//!
//! Add an integer in the range [-k, k] to the element.
//! Return the maximum possible number of distinct elements in nums after performing the operations.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [1,2,2,3,3,4], k = 2
//!
//! Output: 6
//!
//! Explanation:
//!
//! nums changes to [-1, 0, 1, 2, 3, 4] after performing operations on the first four elements.
//!
//! Example 2:
//!
//! Input: nums = [4,4,4,4], k = 1
//!
//! Output: 3
//!
//! Explanation:
//!
//! By adding -1 to nums[0] and 1 to nums[1], nums changes to [3, 5, 4, 4].
//!
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length <= 105
//! 1 <= nums[i] <= 109
//! 0 <= k <= 109

use super::*;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut cnt = 0;
        let mut prev = i32::MIN;

        for num in nums {
            let curr = (num - k).max(prev + 1).min(num + k);
            if curr > prev {
                cnt += 1;
                prev = curr;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test3397() {
        let nums = [1, 2, 2, 3, 3, 4];
        let k = 2;
        let exp = 6;
        let out = Solution::max_distinct_elements(nums.to_vec(), k);
        debug_assert_eq!(exp, out);

        let nums = [4, 4, 4, 4];
        let k = 1;
        let exp = 3;
        let out = Solution::max_distinct_elements(nums.to_vec(), k);
        debug_assert_eq!(exp, out);
    }
}
