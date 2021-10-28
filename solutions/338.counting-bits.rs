/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 */

// @lc code=start
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        (0..=num)                               // create an Iterator from 0 to num
        .map(|x| x.count_ones() as i32)         // use Iterator's map and i32's count_ones function
        .collect::<Vec<i32>>()                  // convert result into Vec<i32>
    }
}
// @lc code=end

