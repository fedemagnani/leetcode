//! You are given two integers, m and k, and an integer array nums.
//!
//! A sequence of integers seq is called magical if:
//! seq has a size of m.
//! 0 <= seq[i] < nums.length
//! The binary representation of 2seq[0] + 2seq[1] + ... + 2seq[m - 1] has k set bits.
//! The array product of this sequence is defined as prod(seq) = (nums[seq[0]] * nums[seq[1]] * ... * nums[seq[m - 1]]).
//!
//! Return the sum of the array products for all valid magical sequences.
//!
//! Since the answer may be large, return it modulo 109 + 7.
//!
//! A set bit refers to a bit in the binary representation of a number that has a value of 1.
//!
//!  
//!
//! Example 1:
//!
//! Input: m = 5, k = 5, nums = [1,10,100,10000,1000000]
//!
//! Output: 991600007
//!
//! Explanation:
//!
//! All permutations of [0, 1, 2, 3, 4] are magical sequences, each with an array product of 1013.
//!
//! Example 2:
//!
//! Input: m = 2, k = 2, nums = [5,4,3,2,1]
//!
//! Output: 170
//!
//! Explanation:
//!
//! The magical sequences are [0, 1], [0, 2], [0, 3], [0, 4], [1, 0], [1, 2], [1, 3], [1, 4], [2, 0], [2, 1], [2, 3], [2, 4], [3, 0], [3, 1], [3, 2], [3, 4], [4, 0], [4, 1], [4, 2], and [4, 3].
//!
//! Example 3:
//!
//! Input: m = 1, k = 1, nums = [28]
//!
//! Output: 28
//!
//! Explanation:
//!
//! The only magical sequence is [0].
//!
//!  
//!
//! Constraints:
//!
//! 1 <= k <= m <= 30
//! 1 <= nums.length <= 50
//! 1 <= nums[i] <= 108

// Notice that 2^seq = 1 << seq
// Notie also that the seq is defined by all the possible permutations of numbers until "nums.length" (excluded). Since 0 is typically included as well, you have a total number of permutations that is nums.length!/(m-nums.length)!
// However, the sequence is considered magical if in the bit representation, the sum of powers has at most k "1". For this, rust has a count_ones() method.
//      The good thing is that permutations involving the same numbers (so, combinations like [1,3,2] and [2,1,3]) can be all discarded if one is considered invalid (because addition is commutative).
//      To compute combinations, you typically compute the combinations of the associated indices. Lucky for us, the elements of the array correspond with their indices (v[i]==i)
// We could first compute the combinations for each num to see if it is a valid combination, and then compute permutations on top of it
// The array "nums" is used only to define the domain of seq and to compute the magic products. We can store it in a u128 type and then compute its modulo with % (1e9 as u128 + 7), returning a i32 type
use super::*;

const MOD: i64 = 1_000_000_007;
use std::collections::HashMap;

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        // Pascal's triangle
        let mut c = vec![vec![0i64; 31]; 31];
        for i in 1..=30 {
            c[i][0] = 1;
            c[i][i] = 1;
            for j in 1..=i / 2 {
                let val = (c[i - 1][j - 1] + c[i - 1][j]) % MOD;
                c[i][j] = val;
                c[i][i - j] = val;
            }
        }

        fn dfs(
            m: i32,
            k: i32,
            i: usize,
            flag: u32,
            nums: &Vec<i32>,
            c: &Vec<Vec<i64>>,
            memo: &mut HashMap<(i32, i32, usize, u32), i64>,
        ) -> i64 {
            if let Some(&res) = memo.get(&(m, k, i, flag)) {
                return res;
            }

            let bz = flag.count_ones() as i32;
            if m < 0 || k < 0 || m + bz < k {
                return 0;
            }
            if m == 0 {
                return if k == bz { 1 } else { 0 };
            }
            if i >= nums.len() {
                return 0;
            }

            let mut ans = 0i64;
            let mut pow_x = 1i64;
            let x = nums[i] as i64;

            for f in 0..=m {
                let perm = (c[m as usize][f as usize] * pow_x) % MOD;
                let new_flag = flag + f as u32;
                let next_flag = new_flag >> 1;
                let bit_set = (new_flag & 1) as i32;
                ans = (ans + perm * dfs(m - f, k - bit_set, i + 1, next_flag, nums, c, memo)) % MOD;
                pow_x = (pow_x * x) % MOD;
            }

            memo.insert((m, k, i, flag), ans);
            ans
        }

        let mut memo = HashMap::new();
        dfs(m, k, 0, 0, &nums, &c, &mut memo) as i32
    }

    pub fn magical_sum_brute(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        let mut sum_array_prod = 0_u128;

        Self::magic_sequences(nums_len, m as usize, k as u32, &mut sum_array_prod, &nums);

        (sum_array_prod % (1e9 as u128 + 7)) as i32
    }

    /// Performs the set_size_check
    fn set_size_check(com: &[usize], k: u32) -> bool {
        let mut out: usize = 0;
        for c in com {
            out += 1 << c;
        }
        out.count_ones() == k
    }

    // Recall that in our case v[i]=i;
    // We compute each combination first, run the set size check, and expand into permutations if succesful
    fn magic_sequences(nums_len: usize, m: usize, k: u32, sum_array_prod: &mut u128, nums: &[i32]) {
        let mut comb: Vec<usize> = vec![0; m];

        loop {
            let check = Self::set_size_check(&comb, k);
            println!("{comb:?} -> {check}");

            if check {
                let array_prod: u128 = comb.iter().fold(1, |acc, i| acc * nums[*i] as u128);
                *sum_array_prod += array_prod; //* fact_m as u128;
            }

            // Computes the combination to be evaluated in next iteration
            // find rightmost element that can be incremented
            let mut i = m;

            while i > 0 {
                i -= 1;
                comb[i] += 1;
                if comb[i] < nums_len {
                    break;
                }
                comb[i] = 0;
            }
            if i == 0 && comb[0] == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test3539() {
        let m = 2;
        let k = 1;
        let nums = [63];
        let exp = 3969;
        let out = Solution::magical_sum(m, k, nums.to_vec());
        debug_assert_eq!(exp, out);

        let m = 5;
        let k = 5;
        let nums = [1, 10, 100, 10000, 1000000];
        let exp = 991600007;
        let out = Solution::magical_sum(m, k, nums.to_vec());
        debug_assert_eq!(exp, out);

        let m = 2;
        let k = 2;
        let nums = [5, 4, 3, 2, 1];
        let exp = 170;
        let out = Solution::magical_sum(m, k, nums.to_vec());
        debug_assert_eq!(exp, out);

        let m = 1;
        let k = 1;
        let nums = [28];
        let exp = 28;
        let out = Solution::magical_sum(m, k, nums.to_vec());
        debug_assert_eq!(exp, out);

        let m = 9;
        let k = 4;
        let nums = [43, 3, 46, 22, 44, 21, 14];
        let exp = 424485515;
        let out = Solution::magical_sum(m, k, nums.to_vec());
        debug_assert_eq!(exp, out);
    }

    #[test]
    fn sol() {
        let sol1 = 1 << 3;
        debug_assert_eq!(sol1, 8);

        let sol2 = 1 << 0;
        debug_assert_eq!(sol2, 1);

        let sol3 = 1 << 4;
        debug_assert_eq!(sol3, 16);

        let sol: i32 = sol1 + sol2 + sol3;
        let set_size = sol.count_ones();
        debug_assert_eq!(set_size, 3);

        debug_assert!(Solution::set_size_check(&[3, 0, 4], 3));
    }

    #[test]
    fn test_modulo() {
        let sol = 1e13 as u128 * 120;
        let sol = sol % (1e9 as u128 + 7);
        debug_assert_eq!(sol, 991600007);
    }
}
