/*
 * @lc app=leetcode id=766 lang=rust
 *
 * [766] Toeplitz Matrix
 */

// @lc code=start
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let l = matrix.len();
        if l == 1 || matrix[0].len() == 1 {
            return true;
        }

        for (i, row) in matrix[..l - 1].iter().enumerate() {
            let next_row_it = matrix[i + 1][1..].iter();
            if row.iter().zip(next_row_it).any(|(x, y)| *x != *y) {
                return false;
            }
        }
        true
    }
}
// @lc code=end

