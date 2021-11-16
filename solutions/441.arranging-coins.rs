/*
 * @lc app=leetcode id=441 lang=rust
 *
 * [441] Arranging Coins
 */

// @lc code=start
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let x = ((0.25 + 2.0 * (n as f64)).sqrt() - 0.5).floor() as i32;
		return x;
    }
}
// @lc code=end

