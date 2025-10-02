//! Given a string s, return the longest palindromic substring in s.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "babad"
//! Output: "bab"
//! Explanation: "aba" is also a valid answer.
//! Example 2:
//!
//! Input: s = "cbbd"
//! Output: "bb"
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 1000
//! s consist of only digits and English letters.

use super::*;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n <= 1 {
            return s;
        }

        let mut best_start = 0;
        let mut best_len = 1;

        // helper closure for expansion
        // we give a chance to each char to be a center

        let mut expand = |mut l: isize, mut r: isize| {
            // we keep widening the interval until we find two characters matching
            // it will continue widening the interval as soon as the palyndrome rule is met
            while l >= 0 && (r as usize) < n && chars[l as usize] == chars[r as usize] {
                l -= 1;
                r += 1;
            }
            // after breaking, palindrome is (l+1 .. r-1)
            let len = (r - l - 1) as usize;
            if len > best_len {
                best_len = len;
                best_start = (l + 1) as usize;
            }
        };

        for i in 0..n {
            expand(i as isize, i as isize); // odd length (aba)
            expand(i as isize, i as isize + 1); // even length (abba)
        }

        chars[best_start..(best_start + best_len)].iter().collect()
    }

    pub fn longest_palindrome_skill_issue_version(s: String) -> String {
        let l = s.len();
        if l == 1 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();

        if l == 2 {
            if chars[0] == chars[1] {
                return s;
            }
            return chars[0].to_string();
        }

        // here we are sure that the string has at least 3 letters

        let mut candidates = vec![];

        for i in 0..l {
            let mut cent: Vec<_>;
            let mut lb;
            let mut ub;

            let mut maybe_center = Vec::with_capacity(l - i);
            let mut end = i;
            for x in chars[i..].iter() {
                if maybe_center.is_empty() {
                    maybe_center.push(*x);
                } else if x == &maybe_center[0] {
                    maybe_center.push(*x);
                    end += 1;
                } else {
                    break;
                }
            }

            if end > i {
                lb = i;
                ub = end;
                cent = maybe_center;
            } else if i > 0 && i < l - 1 {
                let c = chars[i];
                let c_prev = chars[i - 1];
                let c_next = chars[i + 1];

                if c_next != c_prev {
                    continue;
                }
                lb = i - 1;
                ub = i + 1;
                cent = vec![c_prev, c, c_next];
            } else {
                continue;
            }

            if ub >= l {
                candidates.push(cent);
                break;
            }

            let max_iter = lb.min(l - ub - 1);

            for _ in 0..max_iter {
                lb -= 1;
                ub += 1;
                let cl = chars[lb];
                let cu = chars[ub];

                if cl == cu {
                    cent.insert(0, cl);
                    cent.push(cu);
                } else {
                    break;
                }
            }

            candidates.push(cent);
        }

        if candidates.is_empty() {
            return chars[0].to_string();
        }

        candidates
            .iter()
            .max_by_key(|x| x.len())
            .unwrap()
            .iter()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test5() {
        let s = "babad";
        let out = Solution::longest_palindrome(s.to_string());
        debug_assert!(out == "bab" || out == "aba");

        let s = "cbbd";
        let out = Solution::longest_palindrome(s.to_string());
        debug_assert_eq!(out, "bb".to_string());

        let s = "egegewhjqgirafarigpoejenfn";
        let out = Solution::longest_palindrome(s.to_string());
        debug_assert_eq!(out, "girafarig".to_string());

        let s = "ccc";
        let out = Solution::longest_palindrome(s.to_string());
        debug_assert_eq!(out, "ccc".to_string());

        let s = "abb";
        let out = Solution::longest_palindrome(s.to_string());
        debug_assert_eq!(out, "bb".to_string());

        let s = "aaaa";
        let out = Solution::longest_palindrome(s.to_string());
        debug_assert_eq!(out, "aaaa".to_string());
    }
}
