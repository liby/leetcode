/*
 * @lc app=leetcode id=892 lang=rust
 *
 * [892] Surface Area of 3D Shapes
 */

// @lc code=start
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let l = grid.len() as i32;
        let mut res = 0;

        for (row, i) in grid.iter().zip(0..) {
            for (&val, j) in row.iter().zip(0..).filter(|(val, _)| **val > 0) {
                res += 2; // up adn down surfaces
                for (dx, dy) in dirs.iter().cloned() {
                    let (xx, yy) = (j + dx, i + dy);
                    res += match xx >= 0 && yy >= 0 && xx < l && yy < l {
                        true => (val - grid[yy as usize][xx as usize]).max(0),
                        false => val,
                    };
                }
            }
        }
        res
    }
}
// @lc code=end

