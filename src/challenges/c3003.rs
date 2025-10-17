//! You are given a string s and an integer k.
//!
//! First, you are allowed to change at most one index in s to another lowercase English letter.
//!
//! After that, do the following partitioning operation until s is empty:
//!
//! Choose the longest prefix of s containing at most k distinct characters.
//! Delete the prefix from s and increase the number of partitions by one. The remaining characters (if any) in s maintain their initial order.
//! Return an integer denoting the maximum number of resulting partitions after the operations by optimally choosing at most one index to change.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "accca", k = 2
//!
//! Output: 3
//!
//! Explanation:
//!
//! The optimal way is to change s[2] to something other than a and c, for example, b. then it becomes "acbca".
//!
//! Then we perform the operations:
//!
//! The longest prefix containing at most 2 distinct characters is "ac", we remove it and s becomes "bca".
//! Now The longest prefix containing at most 2 distinct characters is "bc", so we remove it and s becomes "a".
//! Finally, we remove "a" and s becomes empty, so the procedure ends.
//! Doing the operations, the string is divided into 3 partitions, so the answer is 3.
//!
//! Example 2:
//!
//! Input: s = "aabaab", k = 3
//!
//! Output: 1
//!
//! Explanation:
//!
//! Initially s contains 2 distinct characters, so whichever character we change, it will contain at most 3 distinct characters, so the longest prefix with at most 3 distinct characters would always be all of it, therefore the answer is 1.
//!
//! Example 3:
//!
//! Input: s = "xxyz", k = 1
//!
//! Output: 4
//!
//! Explanation:
//!
//! The optimal way is to change s[0] or s[1] to something other than characters in s, for example, to change s[0] to w.
//!
//! Then s becomes "wxyz", which consists of 4 distinct characters, so as k is 1, it will divide into 4 partitions.
//!
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 104
//! s consists only of lowercase English letters.
//! 1 <= k <= 26

use super::*;

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let n = s.len();
        let mut left = vec![[0; 3]; n];
        let mut right = vec![[0; 3]; n];
        let bytes = s.as_bytes();

        let (mut num, mut mask, mut count) = (0, 0, 0);
        for i in 0..n - 1 {
            let binary = 1 << (bytes[i] - b'a');
            if mask & binary == 0 {
                count += 1;
                if count <= k {
                    mask |= binary;
                } else {
                    num += 1;
                    mask = binary;
                    count = 1;
                }
            }
            left[i + 1][0] = num;
            left[i + 1][1] = mask;
            left[i + 1][2] = count;
        }

        (num, mask, count) = (0, 0, 0);
        for i in (1..n).rev() {
            let binary = 1 << (bytes[i] - b'a');
            if mask & binary == 0 {
                count += 1;
                if count <= k {
                    mask |= binary;
                } else {
                    num += 1;
                    mask = binary;
                    count = 1;
                }
            }
            right[i - 1][0] = num;
            right[i - 1][1] = mask;
            right[i - 1][2] = count;
        }

        let mut max_val = 0;
        for i in 0..n {
            let mut seg = left[i][0] + right[i][0] + 2;
            let tot_mask = left[i][1] | right[i][1];
            let tot_count = tot_mask.count_ones() as i32;

            if left[i][2] == k && right[i][2] == k && tot_count < 26 {
                seg += 1;
            } else if (tot_count + 1).min(26) <= k {
                seg -= 1;
            }
            max_val = max_val.max(seg);
        }
        max_val
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test3003() {
        let s = "accca";
        let k = 2;
        let exp = 3;
        let out = Solution::max_partitions_after_operations(s.to_string(), k);
        debug_assert_eq!(exp, out);

        let s = "aabaab";
        let k = 3;
        let exp = 1;
        let out = Solution::max_partitions_after_operations(s.to_string(), k);
        debug_assert_eq!(exp, out);

        let s = "xxyz";
        let k = 1;
        let exp = 4;
        let out = Solution::max_partitions_after_operations(s.to_string(), k);
        debug_assert_eq!(exp, out);
    }

    #[test]
    fn t() {
        debug_assert_eq!(b'a', 97);
        debug_assert_eq!(b'z', 122);
    }
}
