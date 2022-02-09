/*
 * @lc app=leetcode id=830 lang=rust
 *
 * [830] Positions of Large Groups
 */

// @lc code=start
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let bytes = s.as_bytes();
        let n = s.len();
        let mut prev: usize = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in (1..n) {
            if bytes[i] != bytes[prev] {
                if i - prev >= 3 {
                    res.push(vec![prev as i32, i as i32 -1]);
                }
                prev = i;
            }
        }
        if n - prev >= 3 {
            res.push(vec![prev as i32, n as i32 -1]);
        }
        return res;
    }
}
// @lc code=end

