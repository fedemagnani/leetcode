//! In a mystic dungeon, n magicians are standing in a line. Each magician has an attribute that gives you energy. Some magicians can give you negative energy, which means taking energy from you.
//!
//! You have been cursed in such a way that after absorbing energy from magician i, you will be instantly transported to magician (i + k). This process will be repeated until you reach the magician where (i + k) does not exist.
//!
//! In other words, you will choose a starting point and then teleport with k jumps until you reach the end of the magicians' sequence, absorbing all the energy during the journey.
//!
//! You are given an array energy and an integer k. Return the maximum possible energy you can gain.
//!
//! Note that when you are reach a magician, you must take energy from them, whether it is negative or positive energy.
//!
//!  
//!
//! Example 1:
//!
//! Input: energy = [5,2,-10,-5,1], k = 3
//!
//! Output: 3
//!
//! Explanation: We can gain a total energy of 3 by starting from magician 1 absorbing 2 + 1 = 3.
//!
//! Example 2:
//!
//! Input: energy = [-2,-3,-1], k = 2
//!
//! Output: -1
//!
//! Explanation: We can gain a total energy of -1 by starting from magician 2.
//!
//!  
//!
//! Constraints:
//!
//! 1 <= energy.length <= 105
//! -1000 <= energy[i] <= 1000
//! 1 <= k <= energy.length - 1

use super::*;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = energy.len();
        let mut out = i32::MIN;

        for i in (n - k)..n {
            let mut i = i;
            let mut acc = 0;

            loop {
                acc += energy[i];
                if acc > out {
                    out = acc;
                }
                if i < k {
                    break;
                }
                i -= k
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test3147() {
        let energy = [5, 2, -10, -5, 1];
        let k = 3;
        let exp = 3;
        let out = Solution::maximum_energy(energy.to_vec(), k);
        debug_assert_eq!(out, exp);

        let energy = [-2, -3, -1];
        let k = 2;
        let exp = -1;
        let out = Solution::maximum_energy(energy.to_vec(), k);
        debug_assert_eq!(out, exp);
    }
}
