//! You are given a 0-indexed array nums consisiting of positive integers. You can do the following operation on the array any number of times:
//!
//! Select an index i such that 0 <= i < n - 1 and replace either of nums[i] or nums[i+1] with their gcd value.
//! Return the minimum number of operations to make all elements of nums equal to 1. If it is impossible, return -1.
//!
//! The gcd of two integers is the greatest common divisor of the two integers.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [2,6,3,4]
//! Output: 4
//! Explanation: We can do the following operations:
//! - Choose index i = 2 and replace nums[2] with gcd(3,4) = 1. Now we have nums = [2,6,1,4].
//! - Choose index i = 1 and replace nums[1] with gcd(6,1) = 1. Now we have nums = [2,1,1,4].
//! - Choose index i = 0 and replace nums[0] with gcd(2,1) = 1. Now we have nums = [1,1,1,4].
//! - Choose index i = 2 and replace nums[3] with gcd(1,4) = 1. Now we have nums = [1,1,1,1].
//!
//! Example 2:
//!
//! Input: nums = [2,10,6,14]
//! Output: -1
//! Explanation: It can be shown that it is impossible to make all the elements equal to 1.
//!  
//!
//! Constraints:
//!
//! 2 <= nums.length <= 50
//! 1 <= nums[i] <= 106

use super::*;

impl Solution {
    pub fn min_operations_2(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a.abs()
        }

        let n = nums.len() as i32;
        let mut num1 = 0;
        let mut g = 0;

        for &x in &nums {
            if x == 1 {
                num1 += 1;
            }
            g = gcd(g, x);
        }

        if num1 > 0 {
            return n - num1;
        }
        if g > 1 {
            return -1;
        }

        let mut min_len = n;
        for i in 0..nums.len() {
            let mut g = 0;
            for (j, num) in nums.iter().enumerate().skip(i) {
                g = gcd(g, *num);
                if g == 1 {
                    min_len = std::cmp::min(min_len, (j - i + 1) as i32);
                    break;
                }
            }
        }

        min_len + n - 2
    }
}
