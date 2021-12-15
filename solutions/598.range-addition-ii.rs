/*
 * @lc app=leetcode id=598 lang=rust
 *
 * [598] Range Addition II
 */

// @lc code=start
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
            return m * n;
        }
        let mut m_max = i32::max_value();
        let mut n_max = i32::max_value();
        for op in ops {
            m_max = std::cmp::min(m_max, op[0]);
            n_max = std::cmp::min(n_max, op[1]);
        }
        return m_max * n_max;
    }
}
// @lc code=end

