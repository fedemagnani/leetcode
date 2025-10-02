//! The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
//!
//! P   A   H   N
//! A P L S I I G
//! Y   I   R
//! And then read line by line: "PAHNAPLSIIGYIR"
//!
//! Write the code that will take a string and make this conversion given a number of rows:
//!
//! string convert(string s, int numRows);
//!  
//!
//! Example 1:
//!
//! Input: s = "PAYPALISHIRING", numRows = 3
//! Output: "PAHNAPLSIIGYIR"
//! Example 2:
//!
//! Input: s = "PAYPALISHIRING", numRows = 4
//! Output: "PINALSIGYAHRPI"
//! Explanation:
//! P     I    N
//! A   L S  I G
//! Y A   H R
//! P     I
//! Example 3:
//!
//! Input: s = "A", numRows = 1
//! Output: "A"
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 1000
//! s consists of English letters (lower-case and upper-case), ',' and '.'.
//! 1 <= numRows <= 1000
use super::*;

// - on the first column, we consume all the chars until the the num_rows-th one
// - then we continue increasing the column and reduce the row-index until the row-index is back to zero
// - then we proceed like we did in the first line.
// - Noticeably, it is needed a matrix of dimension (num_rows, word_len/num_rows + num_rows) to store the zig-zagged word
// - the first square matrix is capable to store 3 *num_rows values, and the others are capable to store 2*num_rows values etch

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        let mut delta_r = false;
        let mut i = 0;

        let mut matrix = vec![String::with_capacity(s.len() / num_rows + 1); num_rows];
        for c in s.chars() {
            matrix[i].push(c);
            if i == 0 || i == num_rows - 1 {
                delta_r = !delta_r;
            }
            if !delta_r {
                i -= 1;
            } else {
                i += 1
            }
        }
        matrix.concat()
    }

    fn cols(mut num_chars: isize, num_rows: isize) -> usize {
        let mut num_cols = num_rows;
        num_chars -= num_rows + 2 * (num_rows - 1);
        while num_chars > 0 {
            num_chars -= 2 * (num_rows - 1);
            num_cols += num_rows - 1;
        }

        num_cols as usize
    }
    pub fn convert_matrix(s: String, num_rows: i32) -> String {
        let l = s.len();
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }
        let num_cols = Self::cols(l as isize, num_rows as isize);

        let mut delta_r: isize = -1;
        let mut i: isize = 0;
        let mut j: isize = 0;

        let mut matrix = vec![vec![None; num_cols]; num_rows];
        let num_rows = num_rows as isize;
        for c in s.chars() {
            matrix[i as usize][j as usize] = Some(c);
            if i == 0 || i == num_rows - 1 {
                delta_r *= -1;
            }
            if delta_r == -1 {
                j += 1;
            }
            i += delta_r;
        }

        matrix.into_iter().flatten().flatten().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test6() {
        let s = "PAYPALISHIRING";
        let num_rows = 3;
        let out = Solution::convert(s.to_string(), num_rows);

        debug_assert_eq!(out, "PAHNAPLSIIGYIR".to_string());

        let s = "AB";
        let num_rows = 1;
        let out = Solution::convert(s.to_string(), num_rows);
        debug_assert_eq!(out, "AB".to_string());

        let s = "ABC";
        let num_rows = 2;
        let out = Solution::convert(s.to_string(), num_rows);
        debug_assert_eq!(out, "ACB".to_string());

        let s = "Apalindromeisaword,phrase,number,orothersequenceofunitsthatcanbereadthesamewayineitherdirection,withgeneralallowancesforadjustmentstopunctuationandworddividers.";
        let num_rows = 3;
        Solution::convert(s.to_string(), num_rows);
    }
}
