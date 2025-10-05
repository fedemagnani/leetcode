//! Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//!
//! Notice that the solution set must not contain duplicate triplets.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [-1,0,1,2,-1,-4]
//! Output: [[-1,-1,2],[-1,0,1]]
//! Explanation:
//! nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//! nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//! nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//! The distinct triplets are [-1,0,1] and [-1,-1,2].
//! Notice that the order of the output and the order of the triplets does not matter.
//! Example 2:
//!
//! Input: nums = [0,1,1]
//! Output: []
//! Explanation: The only possible triplet does not sum up to 0.
//! Example 3:
//!
//! Input: nums = [0,0,0]
//! Output: [[0,0,0]]
//! Explanation: The only possible triplet sums up to 0.
//!
//! Constraints:
//! 3 <= nums.length <= 3000
//! -105 <= nums[i] <= 105

use super::*;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let n = nums.len();
        let mut result = Vec::new();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate `i`
            }

            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    // skip duplicates
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test15() {
        let nums = [-1, 0, 1, 2, -1, -4];
        let exp = [[-1, -1, 2], [-1, 0, 1]];
        let out = Solution::three_sum(nums.to_vec());
        assert_eq!(out.len(), exp.len());
        for o in out {
            exp.iter().find(|x| x.to_vec() == o).unwrap();
        }

        let nums = [0, 1, 1];
        let exp: Vec<Vec<i32>> = Vec::new();
        let out = Solution::three_sum(nums.to_vec());
        assert_eq!(out.len(), exp.len());
        for o in out {
            exp.iter().find(|x| x.to_vec() == o).unwrap();
        }

        let nums = [0, 0, 0];
        let out = Solution::three_sum(nums.to_vec());
        let exp = [[0, 0, 0]];
        assert_eq!(out.len(), exp.len());
        for o in out {
            exp.iter().find(|x| x.to_vec() == o).unwrap();
        }
    }
}
