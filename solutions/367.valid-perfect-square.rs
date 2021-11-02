/*
 * @lc app=leetcode id=367 lang=rust
 *
 * [367] Valid Perfect Square
 */

// @lc code=start
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut l, mut r) = (0, num as i64);
        while l < r {
            let m = l + (r - l) / 2;
            if m * m < num as i64 {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l * l == num as i64
    }
}
// @lc code=end
