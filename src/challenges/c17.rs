//! Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
//!
//! A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
//!
//!
//!  
//!
//! Example 1:
//!
//! Input: digits = "23"
//! Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//! Example 2:
//!
//! Input: digits = "2"
//! Output: ["a","b","c"]
//!  
//!
//! Constraints:
//!
//! 1 <= digits.length <= 4
//! digits[i] is a digit in the range ['2', '9'].

use super::*;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let n = digits.len();

        let phone = [
            vec!["a", "b", "c"],
            vec!["d", "e", "f"],
            vec!["g", "h", "i"],
            vec!["j", "k", "l"],
            vec!["m", "n", "o"],
            vec!["p", "q", "r", "s"],
            vec!["t", "u", "v"],
            vec!["w", "x", "y", "z"],
        ];

        let mut cap = 1;
        let mut indices = Vec::with_capacity(n);
        for b in digits.into_bytes() {
            let i = (b - 50) as usize;
            cap *= phone[i].len();
            indices.push(i);
        }

        let mut out = Vec::with_capacity(cap);

        for l in &phone[indices[0]] {
            out.push(l.to_string());
        }

        for i in indices.into_iter().skip(1) {
            let letters = &phone[i];
            let mut tmp = vec![];
            for d in out.drain(..) {
                for l in letters {
                    tmp.push(format!("{d}{l}"));
                }
            }
            out = tmp
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn utf8() {
        let a = String::from("2");
        let v = a.into_bytes();
        assert_eq!(v[0], 50);
    }

    #[test]
    fn test17() {
        let digits = "2";
        let exp: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let out = Solution::letter_combinations(digits.into());
        assert_eq!(exp, out);

        let digits = "23";
        let exp: Vec<String> = vec![
            "ad".into(),
            "ae".into(),
            "af".into(),
            "bd".into(),
            "be".into(),
            "bf".into(),
            "cd".into(),
            "ce".into(),
            "cf".into(),
        ];
        let out = Solution::letter_combinations(digits.into());
        assert_eq!(exp, out)
    }
}
