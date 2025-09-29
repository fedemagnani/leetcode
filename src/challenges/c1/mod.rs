//! Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//!
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! You can return the answer in any order.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [2,7,11,15], target = 9
//! Output: [0,1]
//! Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//! Example 2:
//!
//! Input: nums = [3,2,4], target = 6
//! Output: [1,2]
//! Example 3:
//!
//! Input: nums = [3,3], target = 6
//! Output: [0,1]
//!
//! Constraints:

//! 2 <= nums.length <= 10^4
//! -10^9 <= nums[i] <= 10^9
//! -10^9 <= target <= 10^9

use std::collections::HashMap;

use super::*;

// Solution is trivial, because by iterating over each candidate, we simply need to find the position of candidate2 = target-candidate1
// Indeed candidate1 + candidate2 = target
// We search the second candidate from the bottom in order to handle cases in which candidate1 = candidate2

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, c1) in nums.iter().enumerate() {
            let c2 = target - c1;
            for (j, num) in nums.iter().enumerate() {
                if j == i {
                    continue;
                }
                if c2.eq(num) {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!()
    }
}

// Alternatively, to reduce complexity, we can cache past iterates and look if they are present.
// We can create a fixed-capacity hashmap that lives on the stack, so that it takes O(1) to check if the iterate is present and it is also cache friendly.
// According to the constraint, the length of nums is maximal 10.000, so we can set this as capacity
// - hashing function: key % capacity
// - conflict resolution: linear probing: next_index = index + 1 % capacity

// Notice that this approach is more memory-hungry than the previous one, however it is quicker (36ms vs 55ms) because time complexitiy is reduced from O(n^2)
// to O(n)

// We pick a SoA approach (struct of array) over an AoS (Array of struct) to reduce chances of a cache miss
// Indeed each cache line is 16 bytes long, meaning that:
// - with AoS, each item is (i32, u16) = 4 bytes + 2 bytes = 6 bytes + padding.
// - with SoA, each key is i32 = 4 bytes + padding
// Thus, this approach gives us more throughput because more that can be cached.
// Moreover, if the values are edited, this doesn't invalidate the key cache line.

#[derive(Copy, Clone, Debug)]
struct TinyMap<const N: usize> {
    keys: [Option<i32>; N], // notice that size(u16) < size(usize) because usize=u64. Moreover u16::max == 65,535 > 1e4 (which is the support for values)
    values: [u16; N], // no need to be optional as they are "guarded" by keys. Getting rid of option let us save 1 byte
}

impl<const N: usize> Default for TinyMap<N> {
    fn default() -> Self {
        Self {
            keys: [None; N],
            values: [0; N],
        }
    }
}

impl<const N: usize> TinyMap<N> {
    /// tries to insert/update a new element
    #[inline(always)]
    fn insert(&mut self, key: i32, value: usize) {
        //using N-1 is faster when N=2^k
        //we upscale numerator by a big constant in order to increase variance of hash function and reduce the risk of probing
        let mut i = (key as u32).wrapping_mul(2654435761) as usize & (N - 1);
        for _ in 0..N {
            let k = &mut self.keys[i]; //unchecked since hash returns always value smaller than capacity
            match k {
                Some(k) => {
                    if key.eq(k) {
                        self.values[i] = value as u16;
                        return;
                    }
                    i = (i + 1) % N; //update hash via linear probing
                }
                None => {
                    self.keys[i] = Some(key);
                    self.values[i] = value as u16;
                    return;
                }
            }
        }
    }

    /// gets the element (if present)
    #[inline(always)]
    fn get(&self, key: i32) -> Option<u16> {
        //using N-1 is faster when N=2^k
        //we upscale numerator by a big constant in order to increase variance of hash function and reduce the risk of probing
        let mut i = (key as u32).wrapping_mul(2654435761) as usize & (N - 1);
        for _ in 0..N {
            let k = self.keys[i]?;
            if key == k {
                return Some(self.values[i]);
            } else {
                i = (i + 1) % N; //update hash via linear probing
            }
        }
        None
    }
}

impl Solution {
    pub fn two_sum_alt(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = TinyMap::<10000>::default();
        for (i, c1) in nums.into_iter().enumerate() {
            let c2 = target - c1;
            let m_j = cache.get(c2);
            let Some(j) = m_j else {
                cache.insert(c1, i); // for future lookups
                continue;
            };
            return vec![i as i32, j as i32];
        }
        panic!()
    }
}

// Compare the solution with a hashmap

impl Solution {
    pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = HashMap::new();
        for (i, c1) in nums.into_iter().enumerate() {
            let c2 = target - c1;
            let m_j = cache.get(&c2);
            let Some(j) = m_j else {
                cache.insert(c1, i as u16); // for future lookups
                continue;
            };
            return vec![i as i32, *j as i32];
        }
        panic!()
    }
}

#[cfg(test)]
mod test {
    use crate::{Solution, challenges::c1::TinyMap};

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let r = Solution::two_sum(nums, target);
        assert_eq!(r, vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;
        let r = Solution::two_sum(nums, target);
        assert_eq!(r, vec![1, 2]);

        let nums = vec![3, 3];
        let target = 6;
        let r = Solution::two_sum(nums, target);
        assert_eq!(r, vec![0, 1]);
    }

    #[test]
    fn tiny_map() {
        let mut map = TinyMap::<10>::default();
        map.insert(1, 42); //hash should be 1
        assert_eq!(map.get(1), Some(42));
        map.insert(1, 17);
        assert_eq!(map.get(1), Some(17));
        map.insert(11, 21); //also here hash should be 1, but we expect linear probing to kick-in
        assert_eq!(map.get(11), Some(21));
        assert_eq!(map.get(1), Some(17));
    }

    #[test]
    fn test1_alt() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let r = Solution::two_sum_alt(nums, target);
        assert!(r == vec![0, 1] || r == vec![1, 0]);

        let nums = vec![3, 2, 4];
        let target = 6;
        let r = Solution::two_sum_alt(nums, target);
        assert!(r == vec![1, 2] || r == vec![2, 1]);

        let nums = vec![3, 3];
        let target = 6;
        let r = Solution::two_sum_alt(nums, target);
        assert!(r == vec![0, 1] || r == vec![1, 0]);
    }

    #[test]
    fn test1_hash() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let r = Solution::two_sum_hash(nums, target);
        assert!(r == vec![0, 1] || r == vec![1, 0]);

        let nums = vec![3, 2, 4];
        let target = 6;
        let r = Solution::two_sum_hash(nums, target);
        assert!(r == vec![1, 2] || r == vec![2, 1]);

        let nums = vec![3, 3];
        let target = 6;
        let r = Solution::two_sum_hash(nums, target);
        assert!(r == vec![0, 1] || r == vec![1, 0]);
    }
}
