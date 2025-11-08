//! You are given a 0-indexed integer array stations of length n, where stations[i] represents the number of power stations in the ith city.
//!
//! Each power station can provide power to every city in a fixed range. In other words, if the range is denoted by r, then a power station at city i can provide power to all cities j such that |i - j| <= r and 0 <= i, j <= n - 1.
//!
//! Note that |x| denotes absolute value. For example, |7 - 5| = 2 and |3 - 10| = 7.
//! The power of a city is the total number of power stations it is being provided power from.
//!
//! The government has sanctioned building k more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.
//!
//! Given the two integers r and k, return the maximum possible minimum power of a city, if the additional power stations are built optimally.
//!
//! Note that you can build the k power stations in multiple cities.
//!
//!  
//!
//! Example 1:
//!
//! Input: stations = [1,2,4,5,0], r = 1, k = 2
//! Output: 5
//! Explanation:
//! One of the optimal ways is to install both the power stations at city 1.
//! So stations will become [1,4,4,5,0].
//! - City 0 is provided by 1 + 4 = 5 power stations.
//! - City 1 is provided by 1 + 4 + 4 = 9 power stations.
//! - City 2 is provided by 4 + 4 + 5 = 13 power stations.
//! - City 3 is provided by 5 + 4 = 9 power stations.
//! - City 4 is provided by 5 + 0 = 5 power stations.
//!
//! So the minimum power of a city is 5.
//! Since it is not possible to obtain a larger power, we return 5.
//! Example 2:
//!
//! Input: stations = [4,4,4,4], r = 0, k = 3
//! Output: 4
//! Explanation:
//! It can be proved that we cannot make the minimum power of a city greater than 4.
//!  
//!
//! Constraints:
//!
//! n == stations.length
//! 1 <= n <= 105
//! 0 <= stations[i] <= 105
//! 0 <= r <= n - 1
//! 0 <= k <= 109

use super::*;

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let mut cnt = vec![0i64; n + 1];

        for (stat, i) in stations.iter().enumerate().take(n) {
            let left = (0).max(i - r) as usize;
            let right = (n as i32).min(i + r + 1) as usize;
            cnt[left] += stat as i64;
            cnt[right] -= stat as i64;
        }

        let min_val = *stations.iter().min().unwrap() as i64;
        let sum_total = stations.iter().map(|&x| x as i64).sum::<i64>();
        let mut hi = sum_total + k as i64;
        let mut lo = min_val;
        let mut res = 0;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if Self::check(&cnt, mid, r, k as i64) {
                res = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        res
    }

    fn check(cnt: &[i64], val: i64, r: i32, k: i64) -> bool {
        let n = cnt.len() - 1;
        let mut diff = cnt.to_vec();
        let mut sum = 0i64;
        let mut remaining = k;

        for i in 0..n {
            sum += diff[i];
            if sum < val {
                let add = val - sum;
                if remaining < add {
                    return false;
                }
                remaining -= add;
                let end = (n as i32).min(i as i32 + 2 * r + 1) as usize;
                diff[end] -= add;
                sum += add;
            }
        }
        true
    }
}
