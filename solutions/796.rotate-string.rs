/*
 * @lc app=leetcode id=796 lang=rust
 *
 * [796] Rotate String
 */

// @lc code=start
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && goal.repeat(2).contains(s.as_str())
    }
}
// @lc code=end

