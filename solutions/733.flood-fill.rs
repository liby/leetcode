/*
 * @lc app=leetcode id=733 lang=rust
 *
 * [733] Flood Fill
 */

// @lc code=start
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let old_color = image[sr as usize][sc as usize];
        if old_color != new_color {
            Self::fill(&mut image, sr as usize, sc as usize, old_color, new_color);
        }
        image
    }

    fn fill(image: &mut Vec<Vec<i32>>, i: usize, j: usize, old_color: i32, new_color: i32) {
        if image[i][j] != old_color {
            return;
        }
        image[i][j] = new_color;
        if i > 0 {
            Self::fill(image, i - 1, j, old_color, new_color);
        }
        if i < image.len() - 1 {
            Self::fill(image, i + 1, j, old_color, new_color);
        }
        if j > 0 {
            Self::fill(image, i, j - 1, old_color, new_color);
        }
        if j < image[0].len() - 1 {
            Self::fill(image, i, j + 1, old_color, new_color);
        }
    }
}
// @lc code=end

