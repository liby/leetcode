/*
 * @lc app=leetcode id=171 lang=rust
 *
 * [171] Excel Sheet Column Number
 */

// @lc code=start
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| (((c as u8 - 64) % 27) as i32 * 26_i32.pow(i as u32)))
            .sum()
    }
}
// @lc code=end
