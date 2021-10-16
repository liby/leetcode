/*
 * @lc app=leetcode id=231 lang=rust
 *
 * [231] Power of Two
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        while n != 1 {
            n = if n % 2 == 0 && n != 0 {
                n / 2
            } else {
                return false;
            };
        }
        true
    }
}
// @lc code=end

