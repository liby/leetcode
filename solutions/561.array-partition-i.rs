/*
 * @lc app=leetcode id=561 lang=rust
 *
 * [561] Array Partition I
 */

// @lc code=start
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums_v = nums;
        nums_v.sort_unstable();
        nums_v.iter().step_by(2).sum()
    }
}
// @lc code=end
