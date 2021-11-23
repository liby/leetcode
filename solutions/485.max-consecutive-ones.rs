/*
 * @lc app=leetcode id=485 lang=rust
 *
 * [485] Max Consecutive Ones
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut res, mut len): (i32, i32) = (0, 0);
        for i in nums {
            if i > 0 {
                len += 1;
            } else {
                res = max(res, len);
                len = 0;
            }
        }
        max(res, len)
    }
}
// @lc code=end

