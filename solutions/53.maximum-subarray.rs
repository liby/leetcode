/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut cur, mut max) = (nums[0], nums[0]);

        for i in 1..nums.len() {
            cur = match cur < 0 {
                true => nums[i],
                false => nums[i] + cur,
            };
            max = max.max(cur);
        }

        max
    }
}
// @lc code=end
