//! Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
//!
//! '.' Matches any single character.​​​​
//! '*' Matches zero or more of the preceding element.
//! The matching should cover the entire input string (not partial).
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "aa", p = "a"
//! Output: false
//! Explanation: "a" does not match the entire string "aa".
//! Example 2:
//!
//! Input: s = "aa", p = "a*"
//! Output: true
//! Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
//! Example 3:
//!
//! Input: s = "ab", p = ".*"
//! Output: true
//! Explanation: ".*" means "zero or more (*) of any character (.)".
//!  
//!
//! Constraints:
//!
//! 1 <= s.length <= 20
//! 1 <= p.length <= 20
//! s contains only lowercase English letters.
//! p contains only lowercase English letters, '.', and '*'.
//! It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.

use super::*;

// the number of dots ints the number of elements we need to skip to verify condition (in other words, we find position and then move |.| dots ahead)
// In presence of .* any number is allowed ()
// if * ends the regex, then the word is allowed to have following chars
//  - if the regex doesn't start or end with*, it means that it is required fixed rules for starting or ending
//  - In this implementation a* and .* are equal
// We could iterate on the regex:
//  - letters are strictly required

use std::collections::HashMap;

impl Solution {
    pub fn is_match(text: String, pattern: String) -> bool {
        let mut memo = HashMap::new();
        Self::dp(&text, &pattern, 0, 0, &mut memo)
    }

    fn dp(
        text: &str,
        pattern: &str,
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        //early stop
        if let Some(&result) = memo.get(&(i, j)) {
            return result;
        }

        let result = if j == pattern.len() {
            //... result is true if we exhausted both the regex pattern and the text
            i == text.len()
        } else {
            //char match
            let first_match = i < text.len()
                && (pattern.as_bytes()[j] == b'.' || pattern.as_bytes()[j] == text.as_bytes()[i]);

            if j + 1 < pattern.len() && pattern.as_bytes()[j + 1] == b'*' {
                // skip 'char*' or match one char and continue with '*'

                // ... it firstly tries to consider the sequence empty, otherwise it iterates over the next char with the same regex rule
                Self::dp(text, pattern, i, j + 2, memo)
                    || (first_match && Self::dp(text, pattern, i + 1, j, memo))
            } else {
                // ... we are here if next regex rule is not *

                //matches the current rule and make progress evaluating the next one
                first_match && Self::dp(text, pattern, i + 1, j + 1, memo)
            }
        };

        memo.insert((i, j), result);
        result
    }
}

#[derive(Debug)]
enum RegexKind {
    Word(u8),
    SequenceOrNone(u8),
    Any,
    AnySequence,
}

impl Solution {
    fn parse_regex(p: &[u8]) -> impl Iterator<Item = RegexKind> {
        let mut out = Vec::with_capacity(p.len());
        let mut p_rev = p.iter().rev();
        while let Some(current) = p_rev.next() {
            if current == &46 {
                //dot
                out.push(RegexKind::Any);
                continue;
            }

            if current == &42 {
                // asterisk
                let next = p_rev.next().unwrap(); //guaranteed by assumption
                if next == &46 {
                    out.push(RegexKind::AnySequence);
                    continue;
                }
                out.push(RegexKind::SequenceOrNone(*next));
                continue;
            }

            out.push(RegexKind::Word(*current));
        }

        out.into_iter().rev()
    }

    pub fn is_match_skill_issue(s: String, p: String) -> bool {
        let p = p.as_bytes();
        let mut reg_rule = Solution::parse_regex(p).peekable();
        let mut s = s.as_bytes().iter().peekable();
        let mut skip_next = false;

        while let Some(rule) = reg_rule.next() {
            if skip_next {
                skip_next = false;
                continue;
            }
            match rule {
                RegexKind::Any => {
                    let c = s.next();
                    if c.is_none() {
                        return false;
                    }
                }
                RegexKind::Word(w) => {
                    let c = s.next();
                    let Some(c) = c else {
                        return false;
                    };
                    if c != &w {
                        return false;
                    }
                }
                RegexKind::SequenceOrNone(w) => {
                    let c = s.peek();
                    let Some(c) = c else {
                        return false;
                    };

                    if c != &&w {
                        continue;
                    }

                    let c = s.next().unwrap();

                    if let Some(RegexKind::Word(y)) | Some(RegexKind::SequenceOrNone(y)) =
                        reg_rule.peek()
                    {
                        skip_next = y == c;
                    }

                    while s.peek() == Some(&&w) {
                        s.next();
                    }
                }
                RegexKind::AnySequence => {
                    let Some(next_rule) = reg_rule.peek() else {
                        while s.next().is_some() {}
                        continue;
                    };

                    match next_rule {
                        RegexKind::SequenceOrNone(x) | RegexKind::Word(x) => {
                            while let Some(t) = s.peek()
                                && t != &x
                            {
                                s.next();
                            }
                        }
                        _ => while s.next().is_some() {},
                    }
                }
            }
        }

        s.next().is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test10() {
        let s = "aaa";
        let p = "ab*a";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(!out);

        let s = "aaa";
        let p = "a*a";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(out);

        let s = "ab";
        let p = ".*c";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(!out);

        let s = "aa";
        let p = "a";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(!out);

        let s = "aa";
        let p = "a*";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(out);

        let s = "ab";
        let p = ".*";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(out);

        let s = "aab";
        let p = "c*a*b";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(out);

        // let s = "mi ss i ss i pp i";
        // let p = "mi s* i s* i p* .";
        let s = "mississippi";
        let p = "mis*is*ip*.";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(out);

        // let s = "mi ss i ss i pp i";
        // let p = "mi s* i s* ? p* .";
        let s = "mississippi";
        let p = "mis*is*p*.";
        let out = Solution::is_match(s.to_string(), p.to_string());
        assert!(!out);
    }

    #[test]
    fn test_regex_parser() {
        let p = "mis*is*ip*..*";
        let mut v = Solution::parse_regex(p.as_bytes());
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::Word(..))); //m
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::Word(..))); //i
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::SequenceOrNone(..))); //s*
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::Word(..))); //i
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::SequenceOrNone(..))); //s*
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::Word(..))); //i
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::SequenceOrNone(..))); //p*
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::Any)); //.
        let n = v.next().unwrap();
        debug_assert!(matches!(n, RegexKind::AnySequence)); //.
    }

    #[test]
    fn bytes_repr() {
        assert_eq!(&[46], ".".as_bytes());
        assert_eq!(&[42], "*".as_bytes());
    }
}
