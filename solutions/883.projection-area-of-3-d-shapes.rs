/*
 * @lc app=leetcode id=883 lang=rust
 *
 * [883] Projection Area of 3D Shapes
 */

// @lc code=start
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut max_in_cols = vec![0; grid.len()];
        for row in &grid {
            let mut max_in_row = 0;
            for (col, &val) in row.iter().enumerate().filter(|(_, val)| **val > 0) {
                max_in_row = max_in_row.max(val);
                max_in_cols[col] = max_in_cols[col].max(val);
                res += 1;
            }
            res += max_in_row;
        }
        res + max_in_cols.iter().sum::<i32>()
    }
}
// @lc code=end

