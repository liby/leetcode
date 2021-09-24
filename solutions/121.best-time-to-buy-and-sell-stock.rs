/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let mut max_profit = 0;
        let mut min_seen = std::i32::MAX;
        // prices.into_iter().for_each(|value| {
        //     max_profit = max(max_profit, value - min_seen);
        //     min_seen = min(min_seen, value);
        // });

        for price in prices {
            min_seen = min(min_seen, price);
            max_profit = max(max_profit, price - min_seen);
        }
        max_profit
    }
}
// @lc code=end
