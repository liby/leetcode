/*
 * @lc app=leetcode id=495 lang=rust
 *
 * [495] Teemo Attacking
 */

// @lc code=start
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        if time_series.len() == 0 {
            return 0;
        }
        let mut res = 0;
        let mut i = 0;
        while i < time_series.len() - 1 {
            if time_series[i + 1] - time_series[i] < duration {
                res += time_series[i + 1] - time_series[i];
            } else {
                res += duration;
            }
            i += 1;
        }
        res + duration
    }
}
// @lc code=end
