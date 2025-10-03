//! Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
//! Output: 4
//! Explanation: After the rain, water is trapped between the blocks.
//! We have two small ponds 1 and 3 units trapped.
//! The total volume of water trapped is 4.
//! Example 2:
//!
//!
//! Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
//! Output: 10
//!  
//!
//! Constraints:
//!
//! m == heightMap.length
//! n == heightMap[i].length
//! 1 <= m, n <= 200
//! 0 <= heightMap[i][j] <= 2 * 104

use super::*;

// element a[i][j] corresponds to the number of the stacked boxes at row i in position j
// Observations:
//  - perimetral positions can't trap any water
//  - we could build the stack of boxes bootom-up and accumulate the number of ponds created
//  - a pond is created if:
//      - position is not perimetral
//      - a "cross" of boxes OR ponds is created around that poisition
//      - it might happen that no ponds are surrounded by a cross of ALL boxes or ALL ponds
//  - the matrix representation gives you a vertical view of the "pool", and you can clearly distinguish by naked eye where ponds are likely to be generated
//
// We can first iterate over the boundary of the "pool", pushing on some stack the cells were the water "could leak" (smallest at the top of the stack):
//  - this is extremely important as boundary cells are upper bound of the water that can be held by the consequent cells
//  - we use a [`BinaryHeap`], pushing the cell value and its coordinate
//  - recall that [`BinaryHeap`] sorts internally via lexicographic order (subsequent values of the tuples are compared only in case of ambiguity on the comparison of fist element of the tuple)
// Then, iterating from the cell with smallest value, we inspect the "cross" around that cell
//  - the "cross" inspection is ordered as follows: down, top, right, left
//  - a matrix maps visited cells, so that algo can skip them
//  - if the neighbor cell is smaller, the result is increased by the difference.
//  - then, the interior cell is put on the [`BinaryHeap`], using as value the maximum between its original value and the compared cell
//      - it keeps the original value (box), if it has a higher value than the compared cell
//      - it uses the compared cell's value (water), if it's original vlaue is lower, meaning that it gets filled of water until allowable value

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();

        if m < 3 || n < 3 {
            return 0;
        }

        let mut visited = vec![vec![false; n]; m];
        let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

        // push all boundary cells
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    heap.push((Reverse(height_map[i][j]), i, j));
                    visited[i][j] = true;
                }
            }
        }

        let dirs: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut res: i32 = 0;

        while let Some((Reverse(h), x, y)) = heap.pop() {
            for (dx, dy) in dirs {
                let nx_isize = x as isize + dx;
                let ny_isize = y as isize + dy;
                if nx_isize < 0 || ny_isize < 0 {
                    continue;
                }
                let nx = nx_isize as usize;
                let ny = ny_isize as usize;
                if nx >= m || ny >= n {
                    continue;
                }
                if visited[nx][ny] {
                    continue;
                }
                visited[nx][ny] = true;

                let nh = height_map[nx][ny];
                let mut diff = 0;
                if nh < h {
                    diff = h - nh;
                    res += diff;
                }
                heap.push((Reverse(nh + diff), nx, ny));
            }
        }

        res
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test407() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 4);

        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 3, 2, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 2, 3, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 5);

        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 9);

        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 10);
    }

    #[test]
    fn hard_case() {
        let height_map = vec![
            vec![12, 13, 1, 12],
            vec![13, 4, 13, 12],
            vec![13, 8, 10, 12],
            vec![12, 13, 12, 12],
            vec![13, 13, 13, 13],
        ];
        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 14);
    }

    #[test]
    fn medium_case_3() {
        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 10);
    }

    #[test]
    fn medium_case_2() {
        let height_map = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 3, 2, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 2, 3, 3],
            vec![3, 3, 3, 3, 3],
        ];

        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 5);
    }

    #[test]
    fn medium_case() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];

        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 4);
    }

    #[test]
    fn easy_case() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 3, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];

        let out = Solution::trap_rain_water(height_map);
        assert_eq!(out, 2)
    }
}
