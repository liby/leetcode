/*
 * @lc app=leetcode id=461 lang=rust
 *
 * [461] Hamming Distance
 */

// @lc code=start
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut distance = 0;
        while x != 0 || y != 0 {
            if (x & 1) != (y & 1) {
                distance += 1;
            }
            x >>= 1;
            y >>= 1;
        }
        distance
    }
}
// @lc code=end

