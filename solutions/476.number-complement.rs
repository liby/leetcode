/*
 * @lc app=leetcode id=476 lang=rust
 *
 * [476] Number Complement
 */

// @lc code=start
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut bits = 0;
        for i in 0..((num as f64).log2().ceil() as i32) {
            bits |= (1 << i);
        }

        !num & bits
    }
}
// @lc code=end

