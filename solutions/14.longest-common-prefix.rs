/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = match strs.get(0) {
            Some(s) => s.to_string(),
            None => String::default(),
        };

        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                match prefix.pop() {
                    Some(_) => continue,
                    None => return prefix,
                }
            }
        }

        prefix
    }
}
// @lc code=end
