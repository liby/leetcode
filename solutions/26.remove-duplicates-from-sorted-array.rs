/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.is_empty() {
            true => 0,
            false => {
                let mut prev = 0;

                for i in 1..nums.len() {
                    if nums[i] != nums[prev] {
                        prev += 1;
                        nums[prev] = nums[i];
                    }
                }

                (prev + 1) as i32
            }
        }
    }
}
// @lc code=end
