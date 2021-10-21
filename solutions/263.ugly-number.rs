/*
 * @lc app=leetcode id=263 lang=rust
 *
 * [263] Ugly Number
 */

// @lc code=start
impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n > 0 {
            for i in 2..6 {
                while (n % i == 0) {
                    n /= i;
                }
            }
        }
        n == 1
    }
}
// @lc code=end

