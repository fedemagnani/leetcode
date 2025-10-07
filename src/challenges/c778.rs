//! You are given an n x n integer matrix grid where each value grid[i][j] represents the elevation at that point (i, j).
//!
//! It starts raining, and water gradually rises over time. At time t, the water level is t, meaning any cell with elevation less than equal to t is submerged or reachable.
//!
//! You can swim from a square to another 4-directionally adjacent square if and only if the elevation of both squares individually are at most t. You can swim infinite distances in zero time. Of course, you must stay within the boundaries of the grid during your swim.
//!
//! Return the minimum time until you can reach the bottom right square (n - 1, n - 1) if you start at the top left square (0, 0).
//!
//!
//!
//! Example 1:
//!
//!
//! Input: grid = [[0,2],[1,3]]
//! Output: 3
//! Explanation:
//! At time 0, you are in grid location (0, 0).
//! You cannot go anywhere else because 4-directionally adjacent neighbors have a higher elevation than t = 0.
//! You cannot reach point (1, 1) until time 3.
//! When the depth of water is 3, we can swim anywhere inside the grid.
//! Example 2:
//!
//!
//! Input: grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
//! Output: 16
//! Explanation: The final route is shown.
//! We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
//!
//!
//! Constraints:
//!
//! n == grid.length
//! n == grid[i].length
//! 1 <= n <= 50
//! 0 <= grid[i][j] < n2
//! Each value grid[i][j] is unique.
use super::*;

// Using a binary heap that allows us to pop always the element with smallest max-observed height, skipping already seen cells, gives us the certainty that the first path reaching top down will be also associated with lowest possible cost
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut visited = vec![false; n * m + m]; //efficient alternative to matrix lookup
        let mut queue = BinaryHeap::with_capacity(n * m);
        queue.push((Reverse(grid[0][0]), 0, 0));

        // we are popping always the cell with lowest cumulative value;
        while let Some((Reverse(highest), i, j)) = queue.pop() {
            // we reached the end
            if i == n - 1 && j == m - 1 {
                return highest;
            }

            for d in Self::DIRS {
                let ni: isize = i as isize + d.0;
                let nj: isize = j as isize + d.1;
                if ni < 0 || nj < 0 {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;

                let v_index = ni * m + nj;
                if ni > n - 1 || nj > m - 1 || visited[v_index] {
                    continue;
                }
                visited[v_index] = true;
                let nh = grid[ni][nj];
                queue.push((Reverse(nh.max(highest)), ni, nj));
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test778() {
        let grid = vec![vec![0, 2], vec![1, 3]];
        let out = Solution::swim_in_water(grid);
        debug_assert_eq!(out, 3);

        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        let out = Solution::swim_in_water(grid);
        debug_assert_eq!(out, 16);
    }
}
