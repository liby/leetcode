/*
 * @lc app=leetcode id=326 lang=rust
 *
 * [326] Power of Three
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        if n > 1 {
            while n % 3 == 0 {
                n /= 3;
            }
        }
        n == 1
    }
}
// @lc code=end

