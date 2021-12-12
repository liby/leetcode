/*
 * @lc app=leetcode id=575 lang=rust
 *
 * [575] Distribute Candies
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let len = candy_type.len();
        let hashset: HashSet<i32> = candy_type.into_iter().collect();
        std::cmp::min(len / 2, hashset.len()) as i32
    }
}
// @lc code=end

