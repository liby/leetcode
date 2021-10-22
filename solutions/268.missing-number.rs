/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (0..=nums.len() as i32).chain(nums.into_iter()).fold(0, |acc, x| acc ^ x)
    }
}
// @lc code=end

