/*
 * @lc app=leetcode id=717 lang=rust
 *
 * [717] 1-bit and 2-bit Characters
 */

// @lc code=start
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let (mut i, last_ind) = (0, bits.len() - 1);
        while i < last_ind {
            i += bits[i] as usize + 1;
        }
        i == last_ind

    }
}
// @lc code=end

