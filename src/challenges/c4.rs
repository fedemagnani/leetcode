//! Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
//!
//! The overall run time complexity should be O(log (m+n)).
//!
//!  
//!
//! Example 1:
//!
//! Input: nums1 = [1,3], nums2 = [2]
//! Output: 2.00000
//! Explanation: merged array = [1,2,3] and median is 2.
//! Example 2:
//!
//! Input: nums1 = [1,2], nums2 = [3,4]
//! Output: 2.50000
//! Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//!  
//!
//! Constraints:
//!
//! nums1.length == m
//! nums2.length == n
//! 0 <= m <= 1000
//! 0 <= n <= 1000
//! 1 <= m + n <= 2000
//! -10^6 <= nums1[i], nums2[i] <= 10^6

use super::*;

// impl Solution {
//     pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//         nums1.extend(nums2.iter());
//         nums1.sort();
//         let l = nums1.len();
//         if !nums1.len().is_multiple_of(2) {
//             nums1[l / 2] as f64
//         } else {
//             (nums1[(l - 1) / 2] + nums1[l / 2]) as f64 / 2.
//         }
//     }
// }

// The fact that arrays are sorted and complexity recommended is log(m+n) hints that optimal approach is binary search.
// The added layer of complexity is that it has to be performed on two different arrays.
// MOreover, differently from binary search, the values is not expected to be inside any of the two arrays

// - We target the smallest array with A, the longest is B
// - |A| = m
// - |B| = n
// - given any index i, we can partition A = a_{0,i} + a_{i,m} (right extremum excluded)
// - we can define an index j = f(i) which partitions B = b_{0,j} + b_{j,n}
//      - j must be such that |a_{0,i}| + |b_{0,j}| = (m+n+1)/2
//      - notice that |a_{0,i}|=i (example: if i=1, then a_{0,i} has only one element since right extremum is excluded)
//      - as a result, from the constraint above, it follows naturally that j = |b_{0,j}| = (m+n+1)/2 - |a_{0,i}| = (m+n+1)/2 - i
// - Since arrays are sorted, it follows naturally that sup[a_{0,i}] <= inf [a_{i,m}] for all i (the same for b)
// - However, it might happen that sup[a_{0,i}] > inf[b_{j,n}]
//      - This hints that the median value is smaller thant the right-most part of a_{0,i}
//      - Indeed, this tells us that we could refine the interval so that the median value is not anymore smaller than the rightmost value of a_{0,i}
//      - As a result, we set the upper bound index of the next iteration as i
// - However, it might happen that sup[b_{0,j}] > inf[a_{i,m}]
//      - This hints that the median value is smaller thant the right-most part of b_{0,j}
//      - Indeed, this tells us that we could refine the interval so that the median value is not anymore smaller than the rightmost value of b_{0,j}
//      - To lower j, we need to increase i of the next iteration, so we set the current i as lower bound for the next iteration;
// - The situation is not furtherly improvable when max(sup[a_{0,i}], sup[b_{0,j}]) <= min(inf [a_{i,m}], inf[b_{j,n}])
//      - In this context, if m+n is odd then the median is guaranteed to be an element of the array, hence it is max(sup[a_{0,i}], sup[b_{0,j}]
//      - if it is even, the median value is the median of the two center elements, hence (max(sup[a_{0,i}], sup[b_{0,j}]) + min(inf [a_{i,m}], inf[b_{j,n}]))/2
//
// - if sup[a_{0,i} \cup |b_{0,j}] <= inf {a_{i,m} \cup |b_{j,n}}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let m: usize = a.len();
        let n = b.len();
        let is_even = (m + n).is_multiple_of(2);

        let mut lb = 0;
        let mut ub = m;

        let mut maxmax = 1000000;
        let mut minmin = -1000000;

        while maxmax > minmin {
            let i = (lb + ub) / 2;
            let j = (m + n).div_ceil(2) - i;

            let a_max = if i == 0 { -1000000 } else { a[i - 1] };
            let b_max = if j == 0 { -1000000 } else { b[j - 1] };

            let a_min = if i == m { 1000000 } else { a[i] };
            let b_min = if j == n { 1000000 } else { b[j] };

            if a_max > b_min {
                ub = i - 1;
            }

            if b_max > a_min {
                lb = i + 1;
            }

            maxmax = a_max.max(b_max);
            minmin = a_min.min(b_min);

            if a_max <= b_min && b_max <= a_min {
                break;
            }
        }

        if is_even {
            return (maxmax + minmin) as f64 / 2.;
        }
        maxmax as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test4() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];

        let s = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(s, 2.);

        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        let s = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(s, 2.5);

        let nums1 = vec![];
        let nums2 = vec![1];

        let s = Solution::find_median_sorted_arrays(nums1, nums2);
        assert_eq!(s, 1.);
    }
}
