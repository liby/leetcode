/*
 * @lc app=leetcode id=482 lang=rust
 *
 * [482] License Key Formatting
 */

// @lc code=start
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        s.to_uppercase()
        .chars()
        .filter(|&char| char != '-')
        .rev()
        .collect::<Vec<char>>()
        .chunks(k as usize)
        .map(|win| win.iter().collect::<String>())
        .collect::<Vec<String>>()
        .as_slice()
        .join("-")
        .chars()
        .rev()
        .collect::<String>()
    }
}
// @lc code=end

