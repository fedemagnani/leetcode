//! You are given a 0-indexed string array words, where words[i] consists of lowercase English letters.
//!
//! In one operation, select any index i such that 0 < i < words.length and words[i - 1] and words[i] are anagrams, and delete words[i] from words. Keep performing this operation as long as you can select an index that satisfies the conditions.
//!
//! Return words after performing all operations. It can be shown that selecting the indices for each operation in any arbitrary order will lead to the same result.
//!
//! An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase using all the original letters exactly once. For example, "dacb" is an anagram of "abdc".
//!
//!  
//!
//! Example 1:
//!
//! Input: words = ["abba","baba","bbaa","cd","cd"]
//! Output: ["abba","cd"]
//! Explanation:
//! One of the ways we can obtain the resultant array is by using the following operations:
//! - Since words[2] = "bbaa" and words[1] = "baba" are anagrams, we choose index 2 and delete words[2].
//!   Now words = ["abba","baba","cd","cd"].
//! - Since words[1] = "baba" and words[0] = "abba" are anagrams, we choose index 1 and delete words[1].
//!   Now words = ["abba","cd","cd"].
//! - Since words[2] = "cd" and words[1] = "cd" are anagrams, we choose index 2 and delete words[2].
//!   Now words = ["abba","cd"].
//!   We can no longer perform any operations, so ["abba","cd"] is the final answer.
//!
//! Example 2:
//!
//! Input: words = ["a","b","c","d","e"]
//! Output: ["a","b","c","d","e"]
//! Explanation:
//! No two adjacent strings in words are anagrams of each other, so no operations are performed.
//!  
//!
//! Constraints:
//!
//! 1 <= words.length <= 100
//! 1 <= words[i].length <= 10
//! words[i] consists of lowercase English letters.

use super::*;

impl Solution {
    #[inline]
    fn is_anagram(a: &str, b: &str) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        if a.len() != b.len() {
            return false;
        }
        let mut prod1: u32 = 1;
        let mut prod2: u32 = 1;
        let mut sum1 = 1;
        let mut sum2 = 1;
        for (a, b) in a.iter().zip(b.iter()) {
            let a = b'z' - *a;
            let b = b'z' - *b;
            sum1 += a;
            sum2 += b;
            prod1 *= a as u32;
            prod2 *= b as u32;
        }
        prod1 == prod2 && sum1 == sum2
    }
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut out = Vec::with_capacity(words.len());
        let mut words_iter = words.into_iter().peekable();
        while let Some(a) = words_iter.next() {
            while let Some(b) = words_iter.peek()
                && Self::is_anagram(&a, b)
            {
                words_iter.next();
            }
            out.push(a);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2273() {
        let words = [
            "abba".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
            "cd".to_string(),
            "cd".to_string(),
        ];
        let exp = ["abba".to_string(), "cd".to_string()];
        let out = Solution::remove_anagrams(words.to_vec());
        debug_assert_eq!(exp.to_vec(), out);

        let words = [
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let exp = [
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let out = Solution::remove_anagrams(words.to_vec());
        debug_assert_eq!(exp.to_vec(), out);
    }

    #[test]
    fn t() {
        let s1 = "rqjrubvt";
        let s2 = "vsuokmjq";
        println!(
            "{:?}",
            s1.as_bytes().iter().map(|x| b'z' - *x).collect::<Vec<_>>()
        );
        println!(
            "{:?}",
            s2.as_bytes().iter().map(|x| b'z' - *x).collect::<Vec<_>>()
        );
        println!(
            "{:?}",
            s1.as_bytes()
                .iter()
                .map(|x| (b'z' - *x) % (b'z' - b'a'))
                .collect::<Vec<_>>()
        );
        println!(
            "{:?}",
            s2.as_bytes()
                .iter()
                .map(|x| (b'z' - *x) % (b'z' - b'a'))
                .collect::<Vec<_>>()
        );
        println!("{}", s1.as_bytes().iter().map(|x| b'z' - *x).sum::<u8>());
        println!("{}", s2.as_bytes().iter().map(|x| b'z' - *x).sum::<u8>());
    }
}
