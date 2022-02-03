/*
 * @lc app=leetcode id=806 lang=rust
 *
 * [806] Number of Lines To Write String
 */

// @lc code=start
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let (mut num_lines, mut last_line) = (0, 0);
        for width in s.as_bytes().iter().map(|b| widths[(*b - b'a') as usize]) {
            last_line += width;
            if last_line > 100 {
                last_line = width;
                num_lines += 1;
            }
        }
        vec![num_lines + 1, last_line]
    }
}
// @lc code=end

