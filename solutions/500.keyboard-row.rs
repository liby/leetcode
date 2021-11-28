/*
 * @lc app=leetcode id=500 lang=rust
 *
 * [500] Keyboard Row
 */

// @lc code=start
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

        words
            .into_iter()
            .filter(|s| {
                rows.iter().fold(false, |b, &row| {
                    b || s.to_lowercase().chars().all(|c| row.contains(c))
                })
            })
            .collect()
    }
}
// @lc code=end
