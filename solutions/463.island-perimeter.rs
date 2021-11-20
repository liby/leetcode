/*
 * @lc app=leetcode id=463 lang=rust
 *
 * [463] Island Perimeter
 */

// @lc code=start
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        let mut prev_row = &vec![];

        for (ind_row, row) in grid.iter().enumerate() {
            let mut prev_ind_col = -2;

            for (ind_col, _) in row.iter().enumerate().filter(|(_, &val)| val == 1) {
                perimeter += 4;

                if ind_row > 0 && prev_row[ind_col] == 1 {
                    perimeter -= 2;
                }

                if (ind_col as i32 - prev_ind_col) == 1 {
                    perimeter -= 2;
                }

                prev_ind_col = ind_col as i32;
            }

            prev_row = row;
        }

        perimeter
    }
}
// @lc code=end

