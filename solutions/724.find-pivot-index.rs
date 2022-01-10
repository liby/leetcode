/*
 * @lc app=leetcode id=724 lang=rust
 *
 * [724] Find Pivot Index
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (i, &val) in nums.iter().enumerate() {
            if total_sum - left_sum - val == left_sum {
                return i as _;
            }
            left_sum += val;
        }
        -1
    }
}
// @lc code=end

