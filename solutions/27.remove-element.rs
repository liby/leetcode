/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        match nums.is_empty() {
            true => 0,
            false => {
                let mut prev = 0;

                for i in 0..nums.len() {
                    if nums[i] != val {
                        nums[prev] = nums[i];
                        prev += 1;
                    }
                }

                prev as i32
            }
        }
    }
}
// @lc code=end
