//! Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, formed from three of these lengths.
//! If it is impossible to form any triangle of a non-zero area, return 0.
//!
//! Example 1:
//!
//! Input: nums = [2,1,2]
//! Output: 5
//! Explanation: You can form a triangle with three side lengths: 1, 2, and 2.
//! Example 2:
//!
//! Input: nums = [1,2,1,10]
//! Output: 0
//! Explanation:
//! You cannot use the side lengths 1, 1, and 2 to form a triangle.
//! You cannot use the side lengths 1, 1, and 10 to form a triangle.
//! You cannot use the side lengths 1, 2, and 10 to form a triangle.
//! As we cannot use any three side lengths to form a triangle of non-zero area, we return 0.
//!  
//!
//! Constraints:
//!
//! 3 <= nums.length <= 104
//! 1 <= nums[i] <= 106

use super::*;

// According to triangular inequality, each side must be smaller than the sum of the other two (a<b+c).
// This is also related to triangular inequality of euclidean norms

// As a result, given a candidate triplet, it is sufficient checking that the biggest length is smaller than the other two:
// this guarantees that that this is valid also for the other two inequalities (it is a system of inequalities)

// We can sort the `nums` vector and create an iterator of overlapping triplets.
// Because of sorting the right-most number will be always the highest one of the triplet.
// As soon as we see a complete triangle iterating from the last (biggest) triplet, we return it since it's guaranteed to be the triangle with biggest perimeter

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        nums.sort();
        for x in nums.windows(3).rev() {
            let sum = x[0] + x[1];
            if sum > x[2] {
                return sum + x[2];
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test976() {
        let nums = vec![2, 1, 2];
        let r = Solution::largest_perimeter(nums);
        assert_eq!(r, 5);

        let nums = vec![1, 2, 1, 10];
        let r = Solution::largest_perimeter(nums);
        assert_eq!(r, 0);
    }
}
