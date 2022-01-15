/*
 * @lc app=leetcode id=744 lang=rust
 *
 * [744] Find Smallest Letter Greater Than Target
 */

// @lc code=start
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let i = letters.binary_search(&target)
            .map(|i| i + 1)
            .unwrap_or_else(|i| i);

        letters[i % letters.len()]
    }
}
// @lc code=end

