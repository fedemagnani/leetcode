//! A magician has various spells.
//!
//! You are given an array power, where each element represents the damage of a spell. Multiple spells can have the same damage value.
//!
//! It is a known fact that if a magician decides to cast a spell with a damage of power[i], they cannot cast any spell with a damage of power[i] - 2, power[i] - 1, power[i] + 1, or power[i] + 2.
//!
//! Each spell can be cast only once.
//!
//! Return the maximum possible total damage that a magician can cast.
//!
//!  
//!
//! Example 1:
//!
//! Input: power = [1,1,3,4]
//!
//! Output: 6
//!
//! Explanation:
//!
//! The maximum possible damage of 6 is produced by casting spells 0, 1, 3 with damage 1, 1, 4.
//!
//! Example 2:
//!
//! Input: power = [7,1,6,6]
//!
//! Output: 13
//!
//! Explanation:
//!
//! The maximum possible damage of 13 is produced by casting spells 1, 2, 3 with damage 1, 6, 6.
//!
//!  
//!
//! Constraints:
//!
//! 1 <= power.length <= 105
//! 1 <= power[i] <= 109

// The caveat here is that if you shoot with a certain spell, you can shoot again the same spell (if present in the array), but not those in the neighborhood [s-2, s+2]/{s}
// we could pair each spell by the maximum times you can shoot it (we iterate once appending values on a hashmap), then iterarting and skip

use super::*;

use std::collections::{BTreeMap, HashMap};
#[derive(Default)]
struct BestDamage {
    low: Option<i64>,
    high: Option<i64>,
}

impl Solution {
    fn inspect_higher(
        current_spell: i32,
        spells: &HashMap<i32, i32>,
        max: i32,
        best_damage: &mut HashMap<i32, BestDamage>,
    ) -> i64 {
        if let Some(Some(high)) = best_damage.get(&current_spell).map(|x| x.high) {
            return high;
        }
        let mut next_higher = current_spell + 3;
        if next_higher > max {
            return 0;
        }
        let mut out = 0;
        while next_higher <= max {
            if !spells.contains_key(&next_higher) {
                next_higher += 1;
                continue;
            }

            let t_h = spells.get(&next_higher).unwrap();

            let inner_out = (next_higher * t_h) as i64
                + Self::inspect_higher(next_higher, spells, max, best_damage);
            if inner_out > out {
                out = inner_out;
            }
            next_higher += 1;
        }

        let damage = best_damage.entry(current_spell).or_default();
        damage.high = Some(out);

        out
    }

    fn inspect_lower(
        current_spell: i32,
        spells: &HashMap<i32, i32>,
        best_damage: &mut HashMap<i32, BestDamage>,
    ) -> i64 {
        if let Some(Some(low)) = best_damage.get(&current_spell).map(|x| x.low) {
            return low;
        }
        let mut next_lower = current_spell - 3;
        if next_lower < 0 {
            return 0;
        }
        let mut out = 0;
        while next_lower > 0 {
            if !spells.contains_key(&next_lower) {
                next_lower -= 1;
                continue;
            }
            let t_l = spells.get(&next_lower).unwrap();

            let inner_out =
                (next_lower * t_l) as i64 + Self::inspect_lower(next_lower, spells, best_damage);
            if inner_out > out {
                out = inner_out;
            }
            next_lower -= 1;
        }

        let damage = best_damage.entry(current_spell).or_default();
        damage.low = Some(out);

        out
    }

    pub fn maximum_total_damage2(power: Vec<i32>) -> i64 {
        let mut spells = HashMap::new();
        let mut max = 0;
        for p in power {
            let entry = spells.entry(p).or_insert(0);
            *entry += 1;
            if p > max {
                max = p
            }
        }

        let mut out = 0;

        let mut cache = HashMap::new(); // cache highest values for future exploration;

        let mut best_damage = HashMap::<i32, BestDamage>::new();

        for (s, t) in &spells {
            let mut inner_out = (s * t) as i64;
            inner_out += Self::inspect_higher(*s, &spells, max, &mut best_damage);
            inner_out += Self::inspect_lower(*s, &spells, &mut best_damage);
            cache.insert(s, inner_out);
            if inner_out > out {
                out = inner_out
            }
        }

        out
    }

    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut count = BTreeMap::new();
        for p in power {
            *count.entry(p).or_insert(0) += 1;
        }
        let mut vec = vec![(-1_000_000_000i32, 0i32)];
        for (k, v) in count {
            vec.push((k, v));
        }
        let n = vec.len();
        let mut f = vec![0i64; n];
        let mut mx = 0i64;
        let mut ans = 0i64;
        let mut j = 1usize;
        for i in 1..n {
            while j < i && vec[j].0 < vec[i].0 - 2 {
                if f[j] > mx {
                    mx = f[j];
                }
                j += 1;
            }
            f[i] = mx + vec[i].0 as i64 * vec[i].1 as i64;
            if f[i] > ans {
                ans = f[i];
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test3186() {
        let power = [1, 1, 3, 4];
        let exp = 6;
        let out = Solution::maximum_total_damage(power.to_vec());
        debug_assert_eq!(exp, out);

        let power = [7, 1, 6, 6];
        let exp = 13;
        let out = Solution::maximum_total_damage(power.to_vec());
        debug_assert_eq!(exp, out);

        let power = [7, 1, 6, 3];
        let exp = 10;
        let out = Solution::maximum_total_damage(power.to_vec());
        debug_assert_eq!(exp, out);

        let power = [5, 9, 2, 10, 2, 7, 10, 9, 3, 8];
        let exp = 31; // = (7*1) + (2*2) + (2*10)
        let out = Solution::maximum_total_damage(power.to_vec());
        debug_assert_eq!(exp, out);
    }
}
