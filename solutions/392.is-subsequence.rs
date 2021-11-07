/*
 * @lc app=leetcode id=392 lang=rust
 *
 * [392] Is Subsequence
 */

// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();
        for c in s.chars() {
            match iter.find(|&p| p == c) {
                Some(_) => (),
                None => return false,
            }
        }
        true
    }
}
// @lc code=end
