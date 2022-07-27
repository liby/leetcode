/*
 * @lc app=leetcode id=896 lang=rust
 *
 * [896] Monotonic Array
 */

// @lc code=start
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        use std::cmp::Ordering;
        let mut dir = 0;
        for (a, b) in nums.iter().zip(nums[1..].iter()) {
            dir |= match a.cmp(b) {
                Ordering::Equal => 0,
                Ordering::Less => 1,
                Ordering::Greater => 2,
            };
            if dir == 3 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

