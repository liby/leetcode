/*
 * @lc app=leetcode id=521 lang=rust
 *
 * [521] Longest Uncommon Subsequence I
 */

// @lc code=start
use core::cmp::max;
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            max(a.len() as i32, b.len() as i32)
        }
    }
}
// @lc code=end

