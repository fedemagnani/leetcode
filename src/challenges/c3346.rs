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
//! Adding 0 to nums[1]. nums becomes [1, 4, 5].
//! Adding -1 to nums[2]. nums becomes [1, 4, 4].
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
//! 1 <= nums[i] <= 105
//! 0 <= k <= 105
//! 0 <= numOperations <= nums.length

// Considering each value as a possible target, we can create a range of values such that, if a generic num is in that range, it can become the target, either because it correspons to the target itself or because we can spend an operation to make it equal
// - Iterate over each value to define a new target
// - Define the range of inclusion
// - Count the elements in the range
// - Update frequency

use super::*;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        // Create the historgram
        let mut hist = [0; 200_005];
        let mut max_num = 0;
        for t in nums {
            if t > max_num {
                max_num = t
            }
            hist[t as usize] += 1
        }

        // Create the CDF
        let mut cdf = hist;
        for i in 1..(max_num + k + 1) {
            let i = i as usize;
            cdf[i] += cdf[i - 1]
        }

        // let min_target = min_num.saturating_sub(k);

        let mut out = 1;
        let k = k as usize;
        for target in 0..max_num + 1 {
            let target = target as usize;
            let lb = target.saturating_sub(k);
            let ub = target + k;

            // we use the cdf for smart count of frequencies;
            // Notice that
            //      cdf[1] = hist[0] + hist[1]
            //      cdf[2] = hist[0] + hist[1] + hist[2]
            //      cdf[4] = hist[0] + hist[1] + hist[2] + hist[3] + hist[4]
            // As a result, the number of frequences in [2,4] is hist[2] + hist[3] + hist[4] = cdf[4]-cdf[1]
            let freq_range = cdf[ub] - cdf[lb.saturating_sub(1)];

            // From the histogram, we can see how many times the target was found;
            let prunable = freq_range - hist[target];

            let freq = hist[target] + prunable.min(num_operations);

            if freq > out {
                out = freq
            }
        }
        out
    }

    pub fn max_frequency_slow(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut out = 0;

        let min_target = nums.iter().min().unwrap() - k;
        let max_target = nums.iter().max().unwrap() + k;

        for target in min_target..=max_target {
            let mut budget = num_operations;
            let lb = target - k;
            let ub = target + k;
            let mut freq = 0;
            for c in nums.iter() {
                if c == &target {
                    freq += 1;
                } else if budget > 0 && c >= &lb && c <= &ub {
                    budget -= 1;
                    freq += 1;
                }
            }
            if freq > out {
                out = freq
            }
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test3346() {
        let nums = [1, 4, 5];
        let k = 1;
        let num_operations = 2;
        let exp = 2;
        let out = Solution::max_frequency(nums.to_vec(), k, num_operations);
        debug_assert_eq!(exp, out);

        let nums = [5, 11, 20, 20];
        let k = 5;
        let num_operations = 1;
        let exp = 2;
        let out = Solution::max_frequency(nums.to_vec(), k, num_operations);
        debug_assert_eq!(exp, out);

        let nums = [88, 53];
        let k = 27;
        let num_operations = 2;
        let exp = 2;
        let out = Solution::max_frequency(nums.to_vec(), k, num_operations);
        debug_assert_eq!(exp, out);

        let nums = [18, 57];
        let k = 97;
        let num_operations = 2;
        let exp = 2;
        let out = Solution::max_frequency(nums.to_vec(), k, num_operations);
        debug_assert_eq!(exp, out);
    }
}
