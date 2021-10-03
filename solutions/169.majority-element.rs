/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

// @lc code=start
impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut count: i32 = 0;
        for i in 0..nums.len() {
            if (i as i32) - 1 == -1 || nums[i - 1] == nums[i] {
                count += 1;
            } else {
                count = 1
            }
            if count > (nums.len() / 2) as i32 {
                return nums[i];
            }
        }
        -1
    }
}
// @lc code=end
