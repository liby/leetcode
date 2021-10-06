/*
 * @lc app=leetcode id=172 lang=rust
 *
 * [172] Factorial Trailing Zeroes
 */

// @lc code=start
impl Solution {
    fn trailing_zeroes_recursive(n: i32, sum: i32) -> i32 {
        if n == 0 {
            sum
        } else {
            Self::trailing_zeroes_recursive(n / 5, sum + n / 5)
        }
    }

    pub fn trailing_zeroes(n: i32) -> i32 {
        Self::trailing_zeroes_recursive(n, 0)
    }
}
// @lc code=end
