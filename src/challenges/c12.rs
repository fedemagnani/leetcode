//! Seven different symbols represent Roman numerals with the following values:
//!
//! Symbol  Value
//! I   1
//! V   5
//! X   10
//! L   50
//! C   100
//! D   500
//! M   1000
//! Roman numerals are formed by appending the conversions of decimal place values from highest to lowest. Converting a decimal place value into a Roman numeral has the following rules:
//!
//! If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input, append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
//! If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol, for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX. Only the following subtractive forms are used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).
//! Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5 (V), 50 (L), or 500 (D) multiple times. If you need to append a symbol 4 times use the subtractive form.
//! Given an integer, convert it to a Roman numeral.
//!
//!  
//!
//! Example 1:
//!
//! Input: num = 3749
//!
//! Output: "MMMDCCXLIX"
//!
//! Explanation:
//!
//! 3000 = MMM as 1000 (M) + 1000 (M) + 1000 (M)
//!  700 = DCC as 500 (D) + 100 (C) + 100 (C)
//!   40 = XL as 10 (X) less of 50 (L)
//!    9 = IX as 1 (I) less of 10 (X)
//! Note: 49 is not 1 (I) less of 50 (L) because the conversion is based on decimal places
//! Example 2:
//!
//! Input: num = 58
//!
//! Output: "LVIII"
//!
//! Explanation:
//!
//! 50 = L
//!  8 = VIII
//! Example 3:
//!
//! Input: num = 1994
//!
//! Output: "MCMXCIV"
//!
//! Explanation:
//!
//! 1000 = M
//!  900 = CM
//!   90 = XC
//!    4 = IV
//!  
//!
//! Constraints:

//! 1 <= num <= 3999

// a naive approach might be creating a huge if/continue block, but it is not ideal since it involves branch prediction.
// instead here we create a cache-friendly SoA and, iterating on each symbol, we deflate the number as much as we can using the i-th symbol

use super::*;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        const VALUES: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        const SYMBOLS: [&str; 13] = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut out = String::with_capacity(15); // preallocate using lenght of longest possible number <= 3999: MMMDCCCLXXXVIII

        for (i, &val) in VALUES.iter().enumerate() {
            //squeeze the number as much as you can
            while num >= val {
                num -= val;
                out.push_str(SYMBOLS[i]);
            }
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test12() {
        let num = 3749;
        let out = Solution::int_to_roman(num);
        debug_assert_eq!(out, "MMMDCCXLIX".to_string());

        let num = 58;
        let out = Solution::int_to_roman(num);
        debug_assert_eq!(out, "LVIII".to_string());

        let num = 1994;
        let out = Solution::int_to_roman(num);
        debug_assert_eq!(out, "MCMXCIV".to_string());
    }
    #[test]
    fn max_roman() {
        let mut len = 0;
        let mut biggest = "".to_string();
        for i in 1..4000 {
            let out = Solution::int_to_roman(i);
            let l = out.len();
            if l > len {
                len = l;
                biggest = out;
            }
        }
        assert_eq!(len, 15);
        assert_eq!(biggest, "MMMDCCCLXXXVIII".to_string());
    }
}
