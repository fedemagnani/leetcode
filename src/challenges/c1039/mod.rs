//! You have a convex n-sided polygon where each vertex has an integer value.
//! You are given an integer array values where values[i] is the value of the ith vertex in clockwise order.
//!
//! Polygon triangulation is a process where you divide a polygon into a set of triangles and the vertices of each triangle must also be vertices of the original polygon.
//! Note that no other shapes other than triangles are allowed in the division. This process will result in n - 2 triangles.
//!
//! You will triangulate the polygon. For each triangle, the weight of that triangle is the product of the values at its vertices. The total score of the triangulation is the sum of these weights over all n - 2 triangles.
//!
//! Return the minimum possible score that you can achieve with some triangulation of the polygon.
//!
//!  
//!
//! Example 1:
//!
//!
//!
//! Input: values = [1,2,3]
//!
//! Output: 6
//!
//! Explanation: The polygon is already triangulated, and the score of the only triangle is 6.
//!
//! Example 2:
//!
//!
//!
//! Input: values = [3,7,4,5]
//!
//! Output: 144
//!
//! Explanation: There are two triangulations, with possible scores: 3*7*5 + 4*5*7 = 245, or 3*4*5 + 3*4*7 = 144.
//! The minimum score is 144.
//!
//! Example 3:
//!
//!
//!
//! Input: values = [1,3,1,4,1,5]
//!
//! Output: 13
//!
//! Explanation: The minimum score triangulation is 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13.
//!
//!  
//!
//! Constraints:
//!
//! n == values.length
//! 3 <= n <= 50
//! 1 <= values[i] <= 100

use super::*;

// The input array can immediately tell us:
// - How many triangles can partition the polygon
// - the weight of each index

// Among all the possible partitions, we need to find that one such that the sum of triangles has minimal weight
// Notice that triangles must create a partition of the figure

// We have [l, ... ,u]: if we take any node "p" in the strict interior of this range (i.e. l < p < u), we can create the following paryition:
// - the triangle {l,p,u}.
// - some polygon (maybe a triangle?) [l, ..., p]
// - some polygon (maybe a triangle?) [p, ..., u]

// As a result, the original problem over the big interval [l, ... ,u] is now decomposed as the same problem but on two smaller subsets: [l, ..., p] and [p, ..., u]

// Notice that if the cardinality is not sufficiently high (i.e. |[a, ..., b]| < 3), then we don't have a polygon, just a segment connecting two nodes (not a single point since k is picked in the strict interior of the interval, so it can't correspond to some extrema)

// As a result, we always expect that b-a >= 2 (not 3 because of zero indeing, indeed: [0,1,2] has cardinality 3), or better (to not cause underflow): b >=2+a

// We need to create a hashmap to store the minimal score given certain intervaal, to avoid expensive computation. We define a hashing method that avoids ambiguity: l*n + u
// Multiplying just the lower bound by the n creates uniquesness, example: (2,5) and (3,4) summed are both 7, but in presence of 10 elements it becomes 25 and 34

/// A square matrix for caching scores on stack.
/// It can be a good alternative to HashMap, saving heap allocations
/// Using maximal capacity of values
struct MatrixCache<const N: usize>([[i32; N]; N]);

impl<const N: usize> MatrixCache<N> {
    const EMPTY_MARKER: i32 = 0;

    #[inline(always)]
    pub const fn new() -> Self {
        let inner = [[Self::EMPTY_MARKER; N]; N];
        Self(inner)
    }

    #[inline(always)]
    pub const fn get(&self, i: usize, j: usize) -> &i32 {
        &self.0[i][j]
    }

    #[inline(always)]
    pub const fn set(&mut self, i: usize, j: usize, score: i32) {
        self.0[i][j] = score
    }

    #[inline(always)]
    pub fn has_score(&self, i: usize, j: usize) -> bool {
        !Self::EMPTY_MARKER.eq(&self.0[i][j])
    }
}

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        // no need to check values length since it's always guaranteed to have at least 3 elements
        let mut cache = MatrixCache::<50>::new();
        Self::subtriangles_score(0, values.len() - 1, &values, &mut cache)
    }

    fn subtriangles_score<const N: usize>(
        l: usize,
        u: usize,
        values: &[i32],
        cache: &mut MatrixCache<N>,
    ) -> i32 {
        let t = l + 2;
        if u < t {
            return 0;
        }

        let k_start = l + 1;

        if u == t {
            return values[l] * values[k_start] * values[u];
        }

        let has_score = cache.has_score(l, u);

        if !has_score {
            let mut best = i32::MAX;
            for k in k_start..u {
                let s = values[l] * values[k] * values[u]
                    + Self::subtriangles_score(l, k, values, cache)
                    + Self::subtriangles_score(k, u, values, cache);
                if s < best {
                    best = s
                }
            }
            cache.set(l, u, best);
        }

        *cache.get(l, u)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1039() {
        let values = vec![1, 2, 3];
        let out = Solution::min_score_triangulation(values);
        assert_eq!(out, 6);

        let values = vec![3, 7, 4, 5];
        let out = Solution::min_score_triangulation(values);
        assert_eq!(out, 144);

        let values = vec![1, 3, 1, 4, 1, 5];
        let out = Solution::min_score_triangulation(values);
        assert_eq!(out, 13);

        let values = vec![2, 1, 4, 4];
        let out = Solution::min_score_triangulation(values);
        assert_eq!(out, 24);

        let values = vec![100; 50];
        let out = Solution::min_score_triangulation(values);
        println!("{out}");
    }
}
