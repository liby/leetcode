/*
 * @lc app=leetcode id=191 lang=rust
 *
 * [191] Number of 1 Bits
 */

// @lc code=start
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        (0..32).map(|num| (n >> num) & 1).fold(0, |x, y| x + y) as i32
    }
}
// @lc code=end
