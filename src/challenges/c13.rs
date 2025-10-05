//! Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//!
//! Symbol       Value
//! I             1
//! V             5
//! X             10
//! L             50
//! C             100
//! D             500
//! M             1000
//! For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
//!
//! Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
//!
//! I can be placed before V (5) and X (10) to make 4 and 9.
//! X can be placed before L (50) and C (100) to make 40 and 90.
//! C can be placed before D (500) and M (1000) to make 400 and 900.
//! Given a roman numeral, convert it to an integer.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "III"
//! Output: 3
//! Explanation: III = 3.
//! Example 2:
//!
//! Input: s = "LVIII"
//! Output: 58
//! Explanation: L = 50, V= 5, III = 3.
//! Example 3:
//!
//! Input: s = "MCMXCIV"
//! Output: 1994
//! Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 15
//! s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
//! It is guaranteed that s is a valid roman numeral in the range [1, 3999].

use super::*;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = [0u16; 22];
        map[0] = 100;
        map[1] = 500;
        map[6] = 1;
        map[9] = 50;
        map[10] = 1000;
        map[19] = 5;
        map[21] = 10;

        let mut last = 0;
        let mut out = 0;
        for c in s.as_bytes().iter().rev() {
            let v = map[(c - b'C') as usize];
            if v >= last {
                out += v
            } else {
                out -= v;
            }
            last = v;
        }
        out as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test13() {
        let s = "III";
        let out = Solution::roman_to_int(s.to_string());
        assert_eq!(out, 3);
        let s = "LVIII";
        let out = Solution::roman_to_int(s.to_string());
        assert_eq!(out, 58);
        let s = "MCMXCIV";
        let out = Solution::roman_to_int(s.to_string());
        assert_eq!(out, 1994);
    }

    #[test]
    fn print_bytes() {
        let out = "MDCLXVI".as_bytes();
        let min = out.iter().min().unwrap();
        assert_eq!(min, &b'C');
        let out = out.iter().map(|x| x - min).collect::<Vec<_>>();
        assert_eq!(out, vec![10, 1, 0, 9, 21, 19, 6]);
    }

    #[test]
    fn t() {
        println!("{}", u16::MAX);
    }
}
