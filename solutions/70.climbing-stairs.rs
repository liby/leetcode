/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut seq = vec![1, 1];

        for i in 1..n {
            seq.push(seq[(i - 1) as usize] + seq[i as usize]);
        }

        seq[n as usize]
    }
}
// @lc code=end
