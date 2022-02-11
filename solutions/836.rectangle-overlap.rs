/*
 * @lc app=leetcode id=836 lang=rust
 *
 * [836] Rectangle Overlap
 */

// @lc code=start
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (x01, y01, x02, y02) = (rec1[0], rec1[1], rec1[2], rec1[3]);
        let (x11, y11, x12, y12) = (rec2[0], rec2[1], rec2[2], rec2[3]);

        !(x01 >= x12 || y01 >= y12 || x02 <= x11 || y02 <= y11)
    }
}
// @lc code=end

