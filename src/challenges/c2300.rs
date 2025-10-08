//! You are given two positive integer arrays spells and potions, of length n and m respectively, where spells[i] represents the strength of the ith spell and potions[j] represents the strength of the jth potion.
//!
//! You are also given an integer success. A spell and potion pair is considered successful if the product of their strengths is at least success.
//!
//! Return an integer array pairs of length n where pairs[i] is the number of potions that will form a successful pair with the ith spell.
//!
//!  
//!
//! Example 1:
//!
//! Input: spells = [5,1,3], potions = [1,2,3,4,5], success = 7
//! Output: [4,0,3]
//! Explanation:
//! - 0th spell: 5 * [1,2,3,4,5] = [5,10,15,20,25]. 4 pairs are successful.
//! - 1st spell: 1 * [1,2,3,4,5] = [1,2,3,4,5]. 0 pairs are successful.
//! - 2nd spell: 3 * [1,2,3,4,5] = [3,6,9,12,15]. 3 pairs are successful.
//!
//! Thus, [4,0,3] is returned.
//! Example 2:
//!
//! Input: spells = [3,1,2], potions = [8,5,8], success = 16
//! Output: [2,0,2]
//! Explanation:
//! - 0th spell: 3 * [8,5,8] = [24,15,24]. 2 pairs are successful.
//! - 1st spell: 1 * [8,5,8] = [8,5,8]. 0 pairs are successful.
//! - 2nd spell: 2 * [8,5,8] = [16,10,16]. 2 pairs are successful.
//!
//! Thus, [2,0,2] is returned.
//!  
//!
//! Constraints:
//!
//! n == spells.length
//! m == potions.length
//! 1 <= n, m <= 105
//! 1 <= spells[i], potions[i] <= 105
//! 1 <= success <= 1010

use super::*;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n = spells.len();
        potions.sort_by(|a, b| b.cmp(a));
        let min = success / *potions.first().unwrap() as i64;

        let mut out = vec![0; n];
        for (i, spell) in spells.iter().enumerate() {
            if (*spell as i64) < min {
                continue;
            }

            // Adjust interval exploiting previous iteration information
            let potions = if i > 0 {
                let prev_i = i - 1;
                let prev_spell = spells[prev_i];
                if spell < &prev_spell {
                    &potions[..out[prev_i] as usize]
                } else if spell > &prev_spell {
                    out[i] = out[prev_i];
                    &potions[out[prev_i] as usize..]
                } else {
                    out[i] = out[prev_i];
                    continue;
                }
            } else {
                &potions
            };

            let mut low = 0;
            let mut high = potions.len();

            while high > low {
                let mid_i = (low + high) / 2;

                let mid = *spell as i64 * potions[mid_i] as i64;

                if success <= mid {
                    low = mid_i + 1;
                } else {
                    high = mid_i;
                }
            }
            out[i] += low as i32
        }

        out
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test2300() {
        let spells = [5, 1, 3];
        let potions = [1, 2, 3, 4, 5];
        let success = 7;
        let exp = [4, 0, 3];
        let out = Solution::successful_pairs(spells.to_vec(), potions.to_vec(), success);
        debug_assert_eq!(out, exp);

        let spells = [3, 1, 2];
        let potions = [8, 5, 8];
        let success = 16;
        let exp = [2, 0, 2];
        let out = Solution::successful_pairs(spells.to_vec(), potions.to_vec(), success);
        debug_assert_eq!(out, exp);

        let spells = [15, 8, 19];
        let potions = [38, 36, 23];
        let success = 328;
        let exp = [3, 0, 3];
        let out = Solution::successful_pairs(spells.to_vec(), potions.to_vec(), success);
        debug_assert_eq!(out, exp);
    }
}
