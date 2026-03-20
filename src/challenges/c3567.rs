//! You are given an m x n integer matrix grid and an integer k.
//!
//! For every contiguous k x k submatrix of grid, compute the minimum absolute difference between any two distinct values within that submatrix.
//!
//! Return a 2D array ans of size (m - k + 1) x (n - k + 1), where ans[i][j] is the minimum absolute difference in the submatrix whose top-left corner is (i, j) in grid.
//!
//! Note: If all elements in the submatrix have the same value, the answer will be 0.
//!
//! A submatrix (x1, y1, x2, y2) is a matrix that is formed by choosing all cells matrix[x][y] where x1 <= x <= x2 and y1 <= y <= y2.
//!  
//!
//! Example 1:
//!
//! Input: grid = [[1,8],[3,-2]], k = 2
//!
//! Output: [[2]]
//!
//! Explanation:
//!
//! There is only one possible k x k submatrix: [[1, 8], [3, -2]].
//! Distinct values in the submatrix are [1, 8, 3, -2].
//! The minimum absolute difference in the submatrix is |1 - 3| = 2. Thus, the answer is [[2]].
//! Example 2:
//!
//! Input: grid = [[3,-1]], k = 1
//!
//! Output: [[0,0]]
//!
//! Explanation:
//!
//! Both k x k submatrix has only one distinct element.
//! Thus, the answer is [[0, 0]].
//! Example 3:
//!
//! Input: grid = [[1,-2,3],[2,3,5]], k = 2
//!
//! Output: [[1,2]]
//!
//! Explanation:
//!
//! There are two possible k × k submatrix:
//! Starting at (0, 0): [[1, -2], [2, 3]].
//! Distinct values in the submatrix are [1, -2, 2, 3].
//! The minimum absolute difference in the submatrix is |1 - 2| = 1.
//! Starting at (0, 1): [[-2, 3], [3, 5]].
//! Distinct values in the submatrix are [-2, 3, 5].
//! The minimum absolute difference in the submatrix is |3 - 5| = 2.
//! Thus, the answer is [[1, 2]].
//!  
//!
//! Constraints:
//!
//! 1 <= m == grid.length <= 30
//! 1 <= n == grid[i].length <= 30
//! -105 <= grid[i][j] <= 105
//! 1 <= k <= min(m, n)

use super::*;

use std::{collections::HashMap, ops::Sub};

struct Cache {
    inner: HashMap<Vec<i32>, i32>,
    tmp: Vec<i32>,
}

impl Cache {
    pub fn new(k: usize) -> Self {
        let inner = HashMap::<Vec<i32>, i32>::new();
        let tmp = Vec::with_capacity(k * k);
        Self { inner, tmp }
    }

    pub fn eval_submatrix(&mut self, grid: &[&[i32]]) -> i32 {
        self.tmp.clear();

        for &el in grid.iter().flat_map(|row| row.iter()) {
            let index = match self.tmp.binary_search(&el) {
                Ok(index) => index,
                Err(index) => index,
            };
            self.tmp.insert(index, el);
        }

        if let Some(&found) = self.inner.get(&self.tmp) {
            self.tmp.clear();
            return found;
        }

        let mut min = i32::MAX;
        for pair in self.tmp.windows(2) {
            let diff = pair[1].sub(pair[0]);
            if diff != 0 {
                min = min.min(diff);
            }
        }

        let ans = if min == i32::MAX { 0 } else { min };
        self.inner.insert(self.tmp.clone(), ans);
        self.tmp.clear();
        ans
    }
}

fn submatrix_iter<T>(
    data: &[T],
    ncols: usize,
    r0: usize,
    r1: usize,
    c0: usize,
    c1: usize,
) -> impl Iterator<Item = &[T]> {
    (r0..=r1).map(move |r| {
        let start = r * ncols + c0;
        &data[start..start + (1 + c1 - c0)]
    })
}

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n_rows = grid.len();
        let n_cols = grid[0].len();
        let k = k as usize;
        let mut cache = Cache::new(k);

        let grid = grid.into_iter().flatten().collect::<Vec<_>>();
        let mut out: Vec<Vec<i32>> = Vec::with_capacity(n_rows - k + 1);
        for i in 0..=n_rows - k {
            let mut out_row: Vec<i32> = Vec::with_capacity(n_cols - k + 1);
            for j in 0..=n_cols - k {
                let (r0, c0) = (i, j);
                let (r1, c1) = (r0 + k - 1, c0 + k - 1);
                let iter = submatrix_iter(&grid, n_cols, r0, r1, c0, c1);
                let submatrix = iter.collect::<Vec<_>>();
                let cell = cache.eval_submatrix(&submatrix);
                out_row.push(cell);
            }
            out.push(out_row);
        }
        out
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_lookup() {
        let grid = vec![&[1, 8][..], &[3, -2][..]];
        let grid2 = vec![&[1, 3][..], &[8, -2][..]];
        let k = 2;
        let mut cache = Cache::new(k);

        let out = cache.eval_submatrix(&grid);
        assert_eq!(out, 2);

        let out = cache.eval_submatrix(&grid2);

        assert_eq!(out, 2);
    }

    #[test]
    fn test_submatrix() {
        let grid = [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]];

        let k = 3;
        let n_rows = grid.len();
        let n_cols = grid[0].len();

        let mut cache = Cache::new(k);

        let grid = grid.into_iter().flatten().collect::<Vec<_>>();
        let mut out: Vec<Vec<i32>> = Vec::with_capacity(n_rows - k + 1);
        for i in 0..=n_rows - k {
            let mut out_row: Vec<i32> = Vec::with_capacity(n_cols - k + 1);
            for j in 0..=n_cols - k {
                let (r0, c0) = (i, j);
                let (r1, c1) = (r0 + k - 1, c0 + k - 1);
                let iter = submatrix_iter(&grid, n_cols, r0, r1, c0, c1);
                let submatrix = iter.collect::<Vec<_>>();
                let cell = cache.eval_submatrix(&submatrix);
                out_row.push(cell);
            }
            out.push(out_row);
        }
    }

    #[test]
    fn test3567() {
        let grid = vec![vec![1, 8], vec![3, -2]];
        let k = 2;
        let expected = vec![vec![2]];

        let out = Solution::min_abs_diff(grid, k);

        assert_eq!(out, expected);

        let grid = vec![vec![3, -1]];
        let k = 1;
        let expected = vec![vec![0, 0]];

        let out = Solution::min_abs_diff(grid, k);

        assert_eq!(out, expected);

        let grid = vec![vec![1, -2, 3], vec![2, 3, 5]];
        let k = 2;
        let expected = vec![vec![1, 2]];

        let out = Solution::min_abs_diff(grid, k);

        assert_eq!(out, expected);
    }
}
