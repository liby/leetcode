/*
 * @lc app=leetcode id=693 lang=rust
 *
 * [693] Binary Number with Alternating Bits
 */

// @lc code=start
impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut prev_bit = -1;
        while n > 0 {
            match n & 1 {
                bit if bit == prev_bit => return false,
                bit => prev_bit = bit,
            }
            n >>= 1;
        }
        true
    }
}
// @lc code=end
