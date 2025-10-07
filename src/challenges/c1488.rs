//! Your country has an infinite number of lakes. Initially, all the lakes are empty, but when it rains over the nth lake, the nth lake becomes full of water. If it rains over a lake that is full of water, there will be a flood. Your goal is to avoid floods in any lake.
//!
//! Given an integer array rains where:
//!
//! rains[i] > 0 means there will be rains over the rains[i] lake.
//! rains[i] == 0 means there are no rains this day and you can choose one lake this day and dry it.
//! Return an array ans where:
//!
//! ans.length == rains.length
//! ans[i] == -1 if rains[i] > 0.
//! ans[i] is the lake you choose to dry in the ith day if rains[i] == 0.
//! If there are multiple valid answers return any of them. If it is impossible to avoid flood return an empty array.
//!
//! Notice that if you chose to dry a full lake, it becomes empty, but if you chose to dry an empty lake, nothing changes.
//!
//!  
//!
//! Example 1:
//!
//! Input: rains = [1,2,3,4]
//! Output: [-1,-1,-1,-1]
//! Explanation: After the first day full lakes are [1]
//! After the second day full lakes are [1,2]
//! After the third day full lakes are [1,2,3]
//! After the fourth day full lakes are [1,2,3,4]
//! There's no day to dry any lake and there is no flood in any lake.
//! Example 2:
//!
//! Input: rains = [1,2,0,0,2,1]
//! Output: [-1,-1,2,1,-1,-1]
//! Explanation: After the first day full lakes are [1]
//! After the second day full lakes are [1,2]
//! After the third day, we dry lake 2. Full lakes are [1]
//! After the fourth day, we dry lake 1. There is no full lakes.
//! After the fifth day, full lakes are [2].
//! After the sixth day, full lakes are [1,2].
//! It is easy that this scenario is flood-free. [-1,-1,1,2,-1,-1] is another acceptable scenario.
//! Example 3:
//!
//! Input: rains = [1,2,0,1,2]
//! Output: []
//! Explanation: After the second day, full lakes are  [1,2]. We have to dry one lake in the third day.
//! After that, it will rain over lakes [1,2]. It's easy to prove that no matter which lake you choose to dry in the 3rd day, the other one will flood.
//!  
//!
//! Constraints:
//!
//! 1 <= rains.length <= 105
//! 0 <= rains[i] <= 109

use super::*;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();
        let mut out = vec![-1; n];

        // Map: lake -> next day it will rain on that lake
        let mut next_rain = HashMap::new();
        let mut next_occurrence = vec![None; n];

        // --- Pass 1: Forecasting (build lookup of next rain day)
        for i in (0..n).rev() {
            let lake = rains[i];
            if lake > 0 {
                if let Some(&next_day) = next_rain.get(&lake) {
                    next_occurrence[i] = Some(next_day);
                }
                next_rain.insert(lake, i);
            }
        }

        // --- Pass 2: Simulation
        let mut full = HashSet::new();
        let mut heap = BinaryHeap::<Reverse<(usize, i32)>>::new();

        for (i, &lake) in rains.iter().enumerate() {
            if lake > 0 {
                // It rains on lake
                if full.contains(&lake) {
                    // Flood — impossible to avoid
                    return vec![];
                }
                full.insert(lake);

                // If we know when it will rain again, push into heap
                if let Some(next_day) = next_occurrence[i] {
                    heap.push(Reverse((next_day, lake)));
                }

                out[i] = -1;
            } else {
                // Dry day — pick the lake that will rain again soonest
                if let Some(Reverse((_, lake_to_dry))) = heap.pop() {
                    full.remove(&lake_to_dry);
                    out[i] = lake_to_dry;
                } else {
                    // No lake needs drying — choose arbitrary lake (e.g., 1)
                    out[i] = 1;
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1488() {
        let rains = [1, 2, 3, 4];
        let out = Solution::avoid_flood(rains.to_vec());
        debug_assert_eq!(out, [-1, -1, -1, -1]);

        let rains = [1, 2, 0, 0, 2, 1];
        let out = Solution::avoid_flood(rains.to_vec());
        debug_assert_eq!(out, [-1, -1, 2, 1, -1, -1]);

        let rains = [1, 2, 0, 1, 2];
        let out = Solution::avoid_flood(rains.to_vec());
        debug_assert_eq!(out, []);

        let rains = [69, 0, 0, 0, 69];
        let out = Solution::avoid_flood(rains.to_vec());
        debug_assert_eq!(out, [-1, 69, 1, 1, -1]);
    }
}
