/*
 * @lc app=leetcode id=389 lang=rust
 *
 * [389] Find the Difference
 */

// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.as_bytes()
            .iter()
            .chain(t.as_bytes())
            .fold(0, |acc, &x| acc ^ x) as char
    }
}
// @lc code=end
