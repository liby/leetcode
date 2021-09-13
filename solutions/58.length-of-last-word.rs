/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // let last_word = s.split_whitespace().last().unwrap();
        // last_word.len() as i32
        match s.split_whitespace().last() {
            Some(word) => word.len() as i32,
            None => 0,
        }
    }
}
// @lc code=end
