/*
 * @lc app=leetcode id=434 lang=rust
 *
 * [434] Number of Segments in a String
 */

// @lc code=start
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut count = 0;
        let mut prev_ch = ' ';
        for ch in s.chars() {
            if ch != ' ' && prev_ch == ' ' {
                count += 1;
            }
            prev_ch = ch;
        }

        count
    }
}
// @lc code=end
