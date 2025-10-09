//! You are given two integer arrays, skill and mana, of length n and m, respectively.
//!
//! In a laboratory, n wizards must brew m potions in order. Each potion has a mana capacity mana[j] and must pass through all the wizards sequentially to be brewed properly. The time taken by the ith wizard on the jth potion is timeij = skill[i] * mana[j].
//!
//! Since the brewing process is delicate, a potion must be passed to the next wizard immediately after the current wizard completes their work. This means the timing must be synchronized so that each wizard begins working on a potion exactly when it arrives. ​
//!
//! Return the minimum amount of time required for the potions to be brewed properly.
//!
//!  
//!
//! Example 1:
//!
//! Input: skill = [1,5,2,4], mana = [5,1,4,2]
//!
//! Output: 110
//!
//! Explanation:
//!
//! Potion Number    Start time    Wizard 0 done by Wizard 1 done by    Wizard 2 done by    Wizard 3 done by
//! 0   0   5   30  40  60
//! 1   52  53  58  60  64
//! 2   54  58  78  86  102
//! 3   86  88  98  102 110
//! As an example for why wizard 0 cannot start working on the 1st potion before time t = 52, consider the case where the wizards started preparing the 1st potion at time t = 50. At time t = 58, wizard 2 is done with the 1st potion, but wizard 3 will still be working on the 0th potion till time t = 60.
//!
//! Example 2:
//!
//! Input: skill = [1,1,1], mana = [1,1,1]
//!
//! Output: 5
//!
//! Explanation:
//!
//! Preparation of the 0th potion begins at time t = 0, and is completed by time t = 3.
//! Preparation of the 1st potion begins at time t = 1, and is completed by time t = 4.
//! Preparation of the 2nd potion begins at time t = 2, and is completed by time t = 5.
//! Example 3:
//!
//! Input: skill = [1,2,3,4], mana = [1,2]
//!
//! Output: 21
//!
//!  
//!
//! Constraints:
//!
//! n == skill.length
//! m == mana.length
//! 1 <= n, m <= 5000
//! 1 <= mana[i], skill[i] <= 5000
use super::*;

// We need to create a "pipeline" of wizards: the first wizard must start when he knows for sure that the last one will be ready to proceed.
// At every new potion, the starting time should behave as follows:
//  - It should be at least the finishing time of the first wizard in the prev iteration (k >= a_i)
//  - the cumulative brewing time until wizard i should be greater than the finishing time of wizard i+1 in the prev iteration (k + m_j * sum_{t=1}^i s_i => a_{i+1}) (if it's smaller, it means that it is finishing before the end of the previous spell)

// As a result, except from the first iteration where it is trivially zero, the starting time of each round is always greater than a_i and  max_{i}{a_{1+1}-m_j * sum_{t=1}^i s_i}

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut prev_iters = [0; 5000];
        let n = skill.len();
        let mut prev_k = 0;
        for m in mana.into_iter() {
            let mut k = prev_iters[0] + prev_k;

            let mut skills_sum = skill[0];
            prev_iters[0] = skills_sum * m;

            for j in 1..n {
                let a_plus = prev_iters[j] + prev_k;
                let t = a_plus - m * skills_sum;
                if t > k {
                    k = t
                }

                skills_sum += skill[j];
                prev_iters[j] = skills_sum * m;
            }
            prev_k = k;
        }

        (prev_iters[n - 1] + prev_k) as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test3494() {
        let skill = [1, 5, 2, 4];
        let mana = [5, 1, 4, 2];
        let exp = 110;
        let out = Solution::min_time(skill.to_vec(), mana.to_vec());
        debug_assert_eq!(out, exp);

        let skill = [1, 1, 1];
        let mana = [1, 1, 1];
        let exp = 5;
        let out = Solution::min_time(skill.to_vec(), mana.to_vec());
        debug_assert_eq!(out, exp);

        let skill = [1, 2, 3, 4];
        let mana = [1, 2];
        let exp = 21;
        let out = Solution::min_time(skill.to_vec(), mana.to_vec());
        debug_assert_eq!(out, exp);

        let skill = [3, 5, 3, 9];
        let mana = [1, 10, 7, 3];
        let exp = 293;
        let out = Solution::min_time(skill.to_vec(), mana.to_vec());
        debug_assert_eq!(out, exp);

        let skill = [5, 4];
        let mana = [3, 2, 6, 1];
        let exp = 85;
        let out = Solution::min_time(skill.to_vec(), mana.to_vec());
        debug_assert_eq!(out, exp);
    }
}
