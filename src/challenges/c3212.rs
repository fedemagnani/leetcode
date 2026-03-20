// Given a 2D character matrix grid, where grid[i][j] is either 'X', 'Y', or '.', return the number of submatrices that contain:

// grid[0][0]
// an equal frequency of 'X' and 'Y'.
// at least one 'X'.

// Example 1:

// Input: grid = [["X","Y","."],["Y",".","."]]

// Output: 3

// Explanation:

// Example 2:

// Input: grid = [["X","X"],["X","Y"]]

// Output: 0

// Explanation:

// No submatrix has an equal frequency of 'X' and 'Y'.

// Example 3:

// Input: grid = [[".","."],[".","."]]

// Output: 0

// Explanation:

// No submatrix has at least one 'X'.

// Constraints:

// 1 <= grid.length, grid[i].length <= 1000
// grid[i][j] is either 'X', 'Y', or '.'.
use super::*;

#[derive(Default, Clone, Copy)]
struct PrefixEntry {
    entry: i32,
    has_x: bool,
}

impl Solution {
    /// We map X=1, Y=-1 and .=0
    ///
    /// Since the requirement is that the submatrix must contain the same number of x and y, the requirement
    /// is asking that the associated submatrix is zero.
    ///
    /// Moreover, since as requirement the sub-matrix must be anchored to grid[0][0], we can use the entries of
    /// the prefix matrix to know the sum of the associated submatrix: if it is zero, then it is a valid candidate
    ///
    /// However, the third requirements asks that there must be at least one 'X' entry in the submatrix:
    /// for this reason we decorate the submatrix entry with a boolean flag which is true either:
    /// - the bottom-right corner of the submatrix is 'X'
    /// - the top sub-submatrix contains 'X'
    /// - the left sub-submatrix contains 'X'
    ///
    /// if the boolean_flag is true and the prefix entry is zero, then we can update the count of valid sub matrices
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut prefix_matrix = vec![vec![PrefixEntry::default(); m]; n];
        let mut out = 0;
        for i in 0..n {
            for j in 0..m {
                let has_above = i > 0;
                let above = if has_above {
                    prefix_matrix[i - 1][j]
                } else {
                    PrefixEntry::default()
                };

                let has_left = j > 0;
                let left = if has_left {
                    prefix_matrix[i][j - 1]
                } else {
                    PrefixEntry::default()
                };

                let diag = if has_above && has_left {
                    prefix_matrix[i - 1][j - 1]
                } else {
                    PrefixEntry::default()
                };
                let a_ij = grid[i][j];
                let is_x = a_ij == 'X';
                let has_x = is_x || above.has_x || left.has_x;
                let a_ij = if is_x {
                    1
                } else if a_ij == 'Y' {
                    -1
                } else {
                    0
                };
                let sum = a_ij + above.entry + left.entry - diag.entry;
                prefix_matrix[i][j].has_x = has_x;
                prefix_matrix[i][j].entry = sum;
                out += (has_x && sum == 0) as i32;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test3212() {
        let grid = vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']];
        let expected = 3;
        let out = Solution::number_of_submatrices(grid);
        assert_eq!(expected, out);

        let grid = vec![vec!['X', 'X'], vec!['X', 'Y']];
        let expected = 0;
        let out = Solution::number_of_submatrices(grid);
        assert_eq!(expected, out);

        let grid = vec![vec!['.', '.'], vec!['.', '.']];
        let expected = 0;
        let out = Solution::number_of_submatrices(grid);
        assert_eq!(expected, out);
    }

    #[test]

    fn t() {
        dbg!(b".", b"X", b"Y");
    }
}
