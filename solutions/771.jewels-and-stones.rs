/*
 * @lc app=leetcode id=771 lang=rust
 *
 * [771] Jewels and Stones
 */

// @lc code=start
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        stones.chars().filter(|&stoun| jewels.contains(stoun)).count() as i32
    }
}
// @lc code=end

