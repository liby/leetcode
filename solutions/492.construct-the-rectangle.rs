/*
 * @lc app=leetcode id=492 lang=rust
 *
 * [492] Construct the Rectangle
 */

// @lc code=start
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut r = 1;
        let mut i = 1;
        while i * i <= area {
            if area % i == 0 {
                r = i;
            }
            i += 1;
        }
        [area / r, r].to_vec()
    }
}
// @lc code=end

