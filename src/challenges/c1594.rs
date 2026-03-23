//! You are given a m x n matrix grid. Initially, you are located at the top-left corner (0, 0), and in each step, you can only move right or down in the matrix.
//!
//! Among all possible paths starting from the top-left corner (0, 0) and ending in the bottom-right corner (m - 1, n - 1), find the path with the maximum non-negative product. The product of a path is the product of all integers in the grid cells visited along the path.
//!
//! Return the maximum non-negative product modulo 109 + 7. If the maximum product is negative, return -1.
//!
//! Notice that the modulo is performed after getting the maximum product.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: grid = [[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]
//! Output: -1
//! Explanation: It is not possible to get non-negative product in the path from (0, 0) to (2, 2), so return -1.
//! Example 2:
//!
//!
//! Input: grid = [[1,-2,1],[1,-2,1],[3,-4,1]]
//! Output: 8
//! Explanation: Maximum non-negative product is shown (1 * 1 * -2 * -4 * 1 = 8).
//! Example 3:
//!
//!
//! Input: grid = [[1,3],[0,-4]]
//! Output: 0
//! Explanation: Maximum non-negative product is shown (1 * 0 * -4 = 0).
//!  
//!
//! Constraints:
//!
//! m == grid.length
//! n == grid[i].length
//! 1 <= m, n <= 15
//! -4 <= grid[i][j] <= 4

use super::*;

#[derive(Default, Clone, Copy)]
struct MaxMin {
    max: i128,
    min: i128,
}
impl MaxMin {
    fn new(a: i128, b: i128) -> Self {
        if a > b {
            return Self { max: a, min: b };
        }
        Self { max: b, min: a }
    }

    fn combine(self, other: Self) -> Self {
        Self {
            max: self.max.max(other.max),
            min: self.min.min(other.min),
        }
    }
}
impl Solution {
    /// Start from the bottom-right cell and go back.
    /// While we traverse the grid from the end, we save for each cell the max and the min
    /// value cumulatively encountered from the end to that cell
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let n_rows = grid.len();
        let n_cols = grid[0].len();
        let mut dp = vec![vec![MaxMin::default(); n_cols]; n_rows];
        let last_row = n_rows - 1;
        let last_col = n_cols - 1;
        let last_cell = grid[last_row][last_col];
        dp[n_rows - 1][n_cols - 1] = MaxMin::new(last_cell as i128, last_cell as i128);
        for i in (0..n_rows).rev() {
            for j in (0..n_cols).rev() {
                let is_last_row = i == last_row;
                let is_last_col = j == last_col;
                // This would happen in the first iteration
                if is_last_row && is_last_col {
                    continue;
                }
                let c = grid[i][j];

                if is_last_row {
                    // we visit only the right neighborhood. It is not granted that
                    // a > b, because the cell might have negative sign
                    let history = dp[i][j + 1];
                    let a = c as i128 * history.max;
                    let b = c as i128 * history.min;
                    dp[i][j] = MaxMin::new(a, b);
                    continue;
                }

                if is_last_col {
                    //we visit only the bootm neighborood
                    let history = dp[i + 1][j];
                    let a = c as i128 * history.max;
                    let b = c as i128 * history.min;
                    dp[i][j] = MaxMin::new(a, b);
                    continue;
                }

                let history_right = dp[i][j + 1];
                let history_down = dp[i + 1][j];

                let a_right = c as i128 * history_right.max;
                let b_right = c as i128 * history_right.min;
                let mm_right = MaxMin::new(a_right, b_right);

                let a_down = c as i128 * history_down.max;
                let b_down = c as i128 * history_down.min;
                let mm_down = MaxMin::new(a_down, b_down);

                dp[i][j] = mm_right.combine(mm_down)
            }
        }

        (dp[0][0].max.max(-1) % (1_000_000_000 + 7)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1594() {
        let grid = vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]];
        let exp = -1;
        let out = Solution::max_product_path(grid);
        assert_eq!(exp, out);

        let grid = vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]];
        let exp = 8;
        let out = Solution::max_product_path(grid);
        assert_eq!(exp, out);

        let grid = vec![vec![1, 3], vec![0, -4]];
        let exp = 0;
        let out = Solution::max_product_path(grid);
        assert_eq!(exp, out);
    }
}
