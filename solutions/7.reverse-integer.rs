/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result = 0i32;
        let mut x = x;

        while x != 0 {
            let tail = x % 10;
            x /= 10;
            if result > i32::MAX / 10 || result < i32::MIN / 10 {
                return 0;
            }
            result = result * 10 + tail;
        }
        result
    }
}
// @lc code=end
