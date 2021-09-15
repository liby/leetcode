/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let mut bound = x as usize;
        let mut base = 0usize;

        while bound >= 2 {
            let half = bound / 2;
            let mid = base + half;

            if mid * mid <= x as usize {
                base = mid;
            }

            bound -= half;
        }

        base as i32
    }
}
// @lc code=end
