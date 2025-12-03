//! You are given a 2D integer array points where points[i] = [xi, yi] represents the coordinates of the ith point on the Cartesian plane.
//!
//! Return the number of unique trapezoids that can be formed by choosing any four distinct points from points.
//!
//! A trapezoid is a convex quadrilateral with at least one pair of parallel sides. Two lines are parallel if and only if they have the same slope.
//!
//!  
//!
//! Example 1:
//!
//! Input: points = [[-3,2],[3,0],[2,3],[3,2],[2,-3]]
//!
//! Output: 2
//!
//! Explanation:
//!
//!
//!
//! There are two distinct ways to pick four points that form a trapezoid:
//!
//! The points [-3,2], [2,3], [3,2], [2,-3] form one trapezoid.
//! The points [2,3], [3,2], [3,0], [2,-3] form another trapezoid.
//! Example 2:
//!
//! Input: points = [[0,0],[1,0],[0,1],[2,1]]
//!
//! Output: 1
//!
//! Explanation:
//!
//!
//!
//! There is only one trapezoid which can be formed.

use super::*;

use std::collections::HashMap;

impl Solution {
    pub fn count_trapezoids_2(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let inf = 1e9 as i32;
        let mut slope_to_intercept: HashMap<String, Vec<String>> = HashMap::new();
        let mut mid_to_slope: HashMap<i64, Vec<String>> = HashMap::new();
        let mut ans = 0;

        for i in 0..n {
            let x1 = points[i][0];
            let y1 = points[i][1];
            for p in points.iter().take(n).skip(i + 1) {
                let x2 = p[0];
                let y2 = p[1];
                let dx = x1 - x2;
                let dy = y1 - y2;

                let (k, b) = if x2 == x1 {
                    (inf.to_string(), x1.to_string())
                } else {
                    let mut k_val = (y2 - y1) as f64 / (x2 - x1) as f64;
                    let mut b_val =
                        (y1 as i64 * dx as i64 - x1 as i64 * dy as i64) as f64 / dx as f64;
                    if k_val == -0.0 {
                        k_val = 0.0;
                    }
                    if b_val == -0.0 {
                        b_val = 0.0;
                    }
                    (format!("{:.10}", k_val), format!("{:.10}", b_val))
                };

                let mid = (x1 + x2) as i64 * 10000 + (y1 + y2) as i64;
                slope_to_intercept
                    .entry(k.clone())
                    .or_default()
                    .push(b.clone());
                mid_to_slope.entry(mid).or_default().push(k);
            }
        }

        for sti in slope_to_intercept.values() {
            if sti.len() == 1 {
                continue;
            }
            let mut cnt: HashMap<&String, i32> = HashMap::new();
            for b_val in sti {
                *cnt.entry(b_val).or_insert(0) += 1;
            }
            let mut total_sum = 0;
            for &count in cnt.values() {
                ans += total_sum * count;
                total_sum += count;
            }
        }

        for mts in mid_to_slope.values() {
            if mts.len() == 1 {
                continue;
            }

            let mut cnt: HashMap<&String, i32> = HashMap::new();
            for k_val in mts {
                *cnt.entry(k_val).or_insert(0) += 1;
            }

            let mut total_sum = 0;
            for &count in cnt.values() {
                ans -= total_sum * count;
                total_sum += count;
            }
        }

        ans
    }
}
