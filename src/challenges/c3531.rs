//! You are given a positive integer n, representing an n x n city. You are also given a 2D grid buildings, where buildings[i] = [x, y] denotes a unique building located at coordinates [x, y].
//!
//! A building is covered if there is at least one building in all four directions: left, right, above, and below.
//!
//! Return the number of covered buildings.
//!
//!  
//!
//! Example 1:
//!
//!
//!
//! Input: n = 3, buildings = [[1,2],[2,2],[3,2],[2,1],[2,3]]
//!
//! Output: 1
//!
//! Explanation:
//!
//! Only building [2,2] is covered as it has at least one building:
//! above ([1,2])
//! below ([3,2])
//! left ([2,1])
//! right ([2,3])
//! Thus, the count of covered buildings is 1.
//! Example 2:
//!
//!
//!
//! Input: n = 3, buildings = [[1,1],[1,2],[2,1],[2,2]]
//!
//! Output: 0
//!
//! Explanation:
//!
//! No building has at least one building in all four directions.
//! Example 3:
//!
//!
//!
//! Input: n = 5, buildings = [[1,3],[3,2],[3,3],[3,5],[5,3]]
//!
//! Output: 1
//!
//! Explanation:
//!
//! Only building [3,3] is covered as it has at least one building:
//! above ([1,3])
//! below ([5,3])
//! left ([3,2])
//! right ([3,5])
//! Thus, the count of covered buildings is 1.
//!  
//!
//! Constraints:
//!
//! 2 <= n <= 105
//! 1 <= buildings.length <= 105
//! buildings[i] = [x, y]
//! 1 <= x, y <= n
//! All coordinates of buildings are unique.

use super::*;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n_usize = n as usize;
        let mut max_row = vec![0; n_usize + 1];
        let mut min_row = vec![n + 1; n_usize + 1];
        let mut max_col = vec![0; n_usize + 1];
        let mut min_col = vec![n + 1; n_usize + 1];

        for p in &buildings {
            let x = p[0] as usize;
            let y = p[1] as usize;

            max_row[y] = max_row[y].max(x as i32);
            min_row[y] = min_row[y].min(x as i32);
            max_col[x] = max_col[x].max(y as i32);
            min_col[x] = min_col[x].min(y as i32);
        }

        let mut res = 0;
        for p in &buildings {
            let x = p[0] as usize;
            let y = p[1] as usize;

            if (x as i32) > min_row[y]
                && (x as i32) < max_row[y]
                && (y as i32) > min_col[x]
                && (y as i32) < max_col[x]
            {
                res += 1;
            }
        }

        res
    }
}
