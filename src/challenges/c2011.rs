//! There is a programming language with only four operations and one variable X:
//!
//! ++X and X++ increments the value of the variable X by 1.
//! --X and X-- decrements the value of the variable X by 1.
//! Initially, the value of X is 0.
//!
//! Given an array of strings operations containing a list of operations, return the final value of X after performing all the operations.
//!
//!  
//!
//! Example 1:
//!
//! Input: operations = ["--X","X++","X++"]
//! Output: 1
//! Explanation: The operations are performed as follows:
//! Initially, X = 0.
//! --X: X is decremented by 1, X =  0 - 1 = -1.
//! X++: X is incremented by 1, X = -1 + 1 =  0.
//! X++: X is incremented by 1, X =  0 + 1 =  1.
//! Example 2:
//!
//! Input: operations = ["++X","++X","X++"]
//! Output: 3
//! Explanation: The operations are performed as follows:
//! Initially, X = 0.
//! ++X: X is incremented by 1, X = 0 + 1 = 1.
//! ++X: X is incremented by 1, X = 1 + 1 = 2.
//! X++: X is incremented by 1, X = 2 + 1 = 3.
//! Example 3:
//!
//! Input: operations = ["X++","++X","--X","X--"]
//! Output: 0
//! Explanation: The operations are performed as follows:
//! Initially, X = 0.
//! X++: X is incremented by 1, X = 0 + 1 = 1.
//! ++X: X is incremented by 1, X = 1 + 1 = 2.
//! --X: X is decremented by 1, X = 2 - 1 = 1.
//! X--: X is decremented by 1, X = 1 - 1 = 0.
//!  
//!
//! Constraints:
//!
//! 1 <= operations.length <= 100
//! operations[i] will be either "++X", "X++", "--X", or "X--".

use super::*;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut out = 0;
        for o in operations.into_iter() {
            let o = o.as_bytes();
            out += (o[1].eq(&b'+') as i8) * 2 - 1;
        }
        out as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2011() {
        let operations = ["--X".to_string(), "X++".to_string(), "X++".to_string()];
        let exp = 1;
        let out = Solution::final_value_after_operations(operations.to_vec());
        debug_assert_eq!(exp, out);

        let operations = ["++X".to_string(), "++X".to_string(), "X++".to_string()];
        let exp = 3;
        let out = Solution::final_value_after_operations(operations.to_vec());
        debug_assert_eq!(exp, out);

        let operations = [
            "X++".to_string(),
            "++X".to_string(),
            "--X".to_string(),
            "X--".to_string(),
        ];
        let exp = 0;
        let out = Solution::final_value_after_operations(operations.to_vec());
        debug_assert_eq!(exp, out);
    }
}
