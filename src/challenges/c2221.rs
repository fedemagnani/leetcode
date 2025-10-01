//! You are given a 0-indexed integer array nums, where nums[i] is a digit between 0 and 9 (inclusive).
//!
//! The triangular sum of nums is the value of the only element present in nums after the following process terminates:
//!
//! Let nums comprise of n elements. If n == 1, end the process. Otherwise, create a new 0-indexed integer array newNums of length n - 1.
//! For each index i, where 0 <= i < n - 1, assign the value of newNums[i] as (nums[i] + nums[i+1]) % 10, where % denotes modulo operator.
//! Replace the array nums with newNums.
//! Repeat the entire process starting from step 1.
//! Return the triangular sum of nums.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: nums = [1,2,3,4,5]
//! Output: 8
//! Explanation:
//! The above diagram depicts the process from which we obtain the triangular sum of the array.
//! Example 2:
//!
//! Input: nums = [5]
//! Output: 5
//! Explanation:
//! Since there is only one element in nums, the triangular sum is the value of that element itself.
//!  
//!
//! Constraints:
//!
//! 1 <= nums.length <= 1000
//! 0 <= nums[i] <= 9

/// The number 9 can be caaptured in 4 bits (1001)
use super::*;

// Struct that wraps two numbers each <=15. The nested value is also hash for the pair that is order-insensitive
#[derive(Debug, Copy, Clone)]
struct PackedPair(u8);

impl PackedPair {
    const fn new(a: u8, b: u8) -> Self {
        if a < b {
            Self(a << 4 | b)
        } else {
            Self(b << 4 | a)
        }
    }
    const fn unwrap(self) -> (u8, u8) {
        let lo = self.0 >> 4;
        let hi = self.0 & 0b00001111;
        (hi, lo)
    }
    const fn hash(&self) -> usize {
        self.0 as usize
    }
    #[inline(always)]
    const fn value(&self) -> u8 {
        let x = self.unwrap();
        (x.0 + x.1) % 10
    }
}

struct Cache([u8; 154]); // 153 == hash of (9,9)

impl Cache {
    const EMPTY_MARKER: u8 = u8::MAX;
    fn new() -> Self {
        Self([Self::EMPTY_MARKER; 154])
    }
    #[inline(always)]
    fn has(&self, p: &PackedPair) -> bool {
        self.0[p.hash()] != Self::EMPTY_MARKER
    }

    #[inline(always)]
    fn get(&self, p: &PackedPair) -> u8 {
        self.0[p.hash()]
    }

    #[inline(always)]
    fn set(&mut self, p: &PackedPair, v: u8) {
        self.0[p.hash()] = v;
    }
}

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums.pop().unwrap();
        }

        let mut cache = Cache::new();
        let nums: Vec<_> = nums
            .windows(2)
            .map(|x| PackedPair::new(x[0] as u8, x[1] as u8))
            .collect();

        let mut current = nums;

        while current.len() > 1 {
            let mut out = vec![];
            for pair in current.windows(2) {
                let pair_a = pair[0];
                let pair_b = pair[1];
                if !cache.has(&pair_a) {
                    cache.set(&pair_a, pair_a.value());
                }
                if !cache.has(&pair_b) {
                    cache.set(&pair_b, pair_b.value());
                }

                let a = cache.get(&pair_a);
                let b = cache.get(&pair_b);

                out.push(PackedPair::new(a, b));
            }

            current = out;
        }

        let pair = current.pop().unwrap();
        if cache.has(&pair) {
            return cache.get(&pair) as i32;
        }
        pair.value() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test2221() {
        let nums = [1, 2, 3, 4, 5];
        let out = Solution::triangular_sum(nums.to_vec());
        assert_eq!(out, 8);

        let nums = [5];
        let out = Solution::triangular_sum(nums.to_vec());
        assert_eq!(out, 5);
    }

    #[test]
    fn packed() {
        let a = 9;
        let b = 6;
        let p1 = PackedPair::new(a, b);

        let x = p1.unwrap();

        assert_eq!(a, x.0);
        assert_eq!(b, x.1);

        let a = 6;
        let b = 9;
        let p2 = PackedPair::new(a, b);

        let x = p2.unwrap();

        assert_eq!(a, x.1);
        assert_eq!(b, x.0);

        assert_eq!(p1.0, p2.0);
    }

    #[test]
    fn max_hash() {
        let a = 9;
        let b = 9;
        let p1 = PackedPair::new(a, b);

        println!("{p1:?}");
    }
}
