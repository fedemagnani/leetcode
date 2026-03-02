//! Given an n x n binary grid, in one step you can choose two adjacent rows of the grid and swap them.
//!
//! A grid is said to be valid if all the cells above the main diagonal are zeros.
//!
//! Return the minimum number of steps needed to make the grid valid, or -1 if the grid cannot be valid.
//!
//! The main diagonal of a grid is the diagonal that starts at cell (1, 1) and ends at cell (n, n).
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: grid = [[0,0,1],[1,1,0],[1,0,0]]
//! Output: 3
//! Example 2:
//!
//!
//! Input: grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]]
//! Output: -1
//! Explanation: All rows are similar, swaps have no effect on the grid.
//! Example 3:
//!
//!
//! Input: grid = [[1,0,0],[1,1,0],[1,1,1]]
//! Output: 0
//!  
//!
//! Constraints:
//!
//! n == grid.length == grid[i].length
//! 1 <= n <= 200
//! grid[i][j] is either 0 or 1

use super::*;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        // Step 1: compute the rightmost 1 for each row
        let mut rightmost = vec![0; n];
        for i in 0..n {
            rightmost[i] = grid[i].iter().rposition(|&x| x == 1).unwrap_or_default()
        }

        let mut swaps = 0;

        // Step 2: process each position
        for pos in 0..n {
            // Find the first row at or below `pos` that can fit here
            let mut j = pos;
            while j < n && rightmost[j] > pos {
                j += 1;
            }

            // No valid row found
            if j == n {
                return -1;
            }

            // Add the number of moves (distance to pos)
            swaps += (j - pos) as i32;

            // Move this row to position `pos` in our mental array
            // We do not need to simulate swapping actual elements; just shift rightmost
            let row = rightmost[j];
            for k in (pos..j).rev() {
                rightmost[k + 1] = rightmost[k];
            }
            rightmost[pos] = row;
        }

        swaps
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1536() {
        let grid = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
        let sol = Solution::min_swaps(grid);
        assert_eq!(sol, 3);

        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
        ];
        let sol = Solution::min_swaps(grid);
        assert_eq!(sol, -1);

        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]];
        let sol = Solution::min_swaps(grid);
        assert_eq!(sol, 0);

        let grid = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let sol = Solution::min_swaps(grid);
        assert_eq!(sol, 2);
    }
}
