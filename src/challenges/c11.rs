//! You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
//!
//! Find two lines that together with the x-axis form a container, such that the container contains the most water.
//!
//! Return the maximum amount of water a container can store.
//!
//! Notice that you may not slant the container.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: height = [1,8,6,2,5,4,8,3,7]
//! Output: 49
//! Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
//! Example 2:
//!
//! Input: height = [1,1]
//! Output: 1
//!  
//!
//! Constraints:
//!
//! n == height.length
//! 2 <= n <= 105
//! 0 <= height[i] <= 104

use super::*;
// we can start from the widest "pool" and restrict the interval.
// The pool’s height corresponds to the smaller of the two extrema. During this search, though, we keep the larger extremum fixed to maintain the possibility that it could define the pool’s height later, if a higher extremum appears.

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max: i32 = 0;
        while i < j {
            let l = height[i];
            let u = height[j];
            let base = (j - i) as i32;
            if l <= u {
                max = (base * l).max(max);
                i += 1;
                continue;
            }
            max = (base * u).max(max);
            j -= 1;
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test11() {
        let t = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let out = Solution::max_area(t);
        assert_eq!(out, 49);
        let t = vec![1, 1];
        let out = Solution::max_area(t);
        assert_eq!(out, 1);
    }
}
