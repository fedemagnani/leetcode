//! There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
//!
//! The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
//!
//! The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
//!
//! Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
//! Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
//! Explanation: The following cells can flow to the Pacific and Atlantic oceans, as shown below:
//! [0,4]: [0,4] -> Pacific Ocean
//!        [0,4] -> Atlantic Ocean
//! [1,3]: [1,3] -> [0,3] -> Pacific Ocean
//!        [1,3] -> [1,4] -> Atlantic Ocean
//! [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean
//!        [1,4] -> Atlantic Ocean
//! [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean
//!        [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
//! [3,0]: [3,0] -> Pacific Ocean
//!        [3,0] -> [4,0] -> Atlantic Ocean
//! [3,1]: [3,1] -> [3,0] -> Pacific Ocean
//!        [3,1] -> [4,1] -> Atlantic Ocean
//! [4,0]: [4,0] -> Pacific Ocean
//!        [4,0] -> Atlantic Ocean
//! Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
//! Example 2:
//!
//! Input: heights = [[1]]
//! Output: [[0,0]]
//! Explanation: The water can flow from the only cell to the Pacific and Atlantic oceans.
//!  
//!
//! Constraints:
//!
//! m == heights.length
//! n == heights[r].length
//! 1 <= m, n <= 200
//! 0 <= heights[r][c] <= 105

use super::*;

// This problem can be in principle similiar to the problem where we were trapping rain in pods (c407)
// We can explore the boundary using a binaryheap.
// A necessary and sufficient condition for the existence of a path connecting the two oceans is a path that traverses the graph passing through the diagonal. This implies that
//  - both source and sink node live at the boundary of the matrix
//  - source and sink node live in the opposite sides of the matrix
//  - at least one cell lives on the main diagonal
//  - elements at the extremum of the diagonal are always valid paths connecting the two oceans
//  - if a cell is higher than a neighbor which is also a peek, this is sufficient (but not necessary condition) for being a peek
//  - Necessary (but not sufficient) condition for being a peek is that you are bigger of at least one of your neighbors (unless you are at the extrmum of the diagonal)
//  - Sufficient (but not necessary) condition for being a peek is that you are the biggest of your row or column

// Naive aprroach is tracking bacwardly all the routes from pacific, then all the routes from atlantic and see the intersection.

//We push three elements: the two coordinates and a bool, to flag is the candidate is pacific or not
#[derive(Clone, Copy, Default)]
struct Visited {
    atlantic: bool,
    pacific: bool,
}
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let m = heights[0].len();

        let mut visited = vec![Visited::default(); m * n + m];

        let mut out = Vec::with_capacity(n * m);
        let mut candidates = Vec::with_capacity(n * m);

        // we set the boundaries as candidate
        for i in 0..n {
            candidates.push((i, 0, true));
            candidates.push((i, m - 1, false));
        }
        for j in 0..m {
            candidates.push((0, j, true));
            candidates.push((n - 1, j, false));
        }

        while let Some((i, j, is_pacific)) = candidates.pop() {
            let check = &mut visited[i * m + j];
            let (v, compare) = if is_pacific {
                (&mut check.pacific, check.atlantic)
            } else {
                (&mut check.atlantic, check.pacific)
            };

            if *v {
                continue;
            }
            *v = true;

            if compare {
                out.push(vec![i as i32, j as i32]);
            }
            // top
            if i > 0 && heights[i - 1][j] >= heights[i][j] {
                candidates.push((i - 1, j, is_pacific));
            }
            // down
            if i < n - 1 && heights[i + 1][j] >= heights[i][j] {
                candidates.push((i + 1, j, is_pacific));
            }
            // left
            if j > 0 && heights[i][j - 1] >= heights[i][j] {
                candidates.push((i, j - 1, is_pacific));
            }
            // right
            if j < m - 1 && heights[i][j + 1] >= heights[i][j] {
                candidates.push((i, j + 1, is_pacific));
            }
        }

        out
    }

    pub fn run_exploration(
        candidates: &mut Vec<(usize, usize)>,
        heights: &[Vec<i32>],
        n: usize,
        m: usize,
    ) -> Vec<Vec<bool>> {
        let mut visited = vec![vec![false; m]; n];

        while let Some((i, j)) = candidates.pop() {
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            // top
            if i > 0 && heights[i - 1][j] >= heights[i][j] {
                candidates.push((i - 1, j));
            }
            // down
            if i < n - 1 && heights[i + 1][j] >= heights[i][j] {
                candidates.push((i + 1, j));
            }
            // left
            if j > 0 && heights[i][j - 1] >= heights[i][j] {
                candidates.push((i, j - 1));
            }
            // right
            if j < m - 1 && heights[i][j + 1] >= heights[i][j] {
                candidates.push((i, j + 1));
            }
        }
        visited
    }

    pub fn pacific_atlantic_2(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let m = heights[0].len();

        let mut pac_candidates = vec![];
        let mut atl_candidates = vec![];

        // we set the boundaries as candidate
        for i in 0..n {
            pac_candidates.push((i, 0));
            atl_candidates.push((i, m - 1));
        }
        for j in 0..m {
            pac_candidates.push((0, j));
            atl_candidates.push((n - 1, j));
        }

        let pac_visited = Self::run_exploration(&mut pac_candidates, &heights, n, m);
        let atl_visited = Self::run_exploration(&mut atl_candidates, &heights, n, m);

        let mut out = vec![];
        for i in 0..n {
            for j in 0..m {
                if pac_visited[i][j] && atl_visited[i][j] {
                    out.push(vec![i as i32, j as i32]);
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test417() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];

        let out = Solution::pacific_atlantic(heights);

        let expected = [
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];

        for e in expected {
            out.iter().find(|o| o == &&e).unwrap();
        }
    }
}
