/*
 * @lc app=leetcode id=832 lang=rust
 *
 * [832] Flipping an Image
 */

// @lc code=start
impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for v in image.iter_mut() {
            v.reverse();
            for l in v.iter_mut() {
                *l ^= 1;
            }
        }
        image
    }
}
// @lc code=end

