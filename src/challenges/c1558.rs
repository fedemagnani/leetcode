//! There are numBottles water bottles that are initially full of water. You can exchange numExchange empty water bottles from the market with one full water bottle.
//!
//! The operation of drinking a full water bottle turns it into an empty bottle.
//!
//! Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.
//!
//!  
//!
//! Example 1:
//!
//!
//! Input: numBottles = 9, numExchange = 3
//! Output: 13
//! Explanation: You can exchange 3 empty bottles to get 1 full water bottle.
//! Number of water bottles you can drink: 9 + 3 + 1 = 13.
//! Example 2:
//!
//!
//! Input: numBottles = 15, numExchange = 4
//! Output: 19
//! Explanation: You can exchange 4 empty bottles to get 1 full water bottle.
//! Number of water bottles you can drink: 15 + 3 + 1 = 19.
//!  
//!
//! Constraints:
//!
//! 1 <= numBottles <= 100
//! 2 <= numExchange <= 100

use super::*;

impl Solution {
    pub fn num_water_bottles(full: i32, num_exchange: i32) -> i32 {
        let mut full = full as u8;
        let num_exchange = num_exchange as u8;
        let mut drank = 0;
        let mut empty = 0;

        while full > 0 {
            drank += full;
            empty += full;
            full = empty / num_exchange; // exchanged bottles
            empty %= num_exchange; // leftover empties
        }
        drank as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1558() {
        let num_bottles = 9;
        let num_exchange = 3;
        let out = Solution::num_water_bottles(num_bottles, num_exchange);

        debug_assert_eq!(out, 13);

        let num_bottles = 15;
        let num_exchange = 4;
        let out = Solution::num_water_bottles(num_bottles, num_exchange);

        debug_assert_eq!(out, 19);
    }
}
