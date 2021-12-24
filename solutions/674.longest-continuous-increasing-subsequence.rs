/*
 * @lc app=leetcode id=674 lang=rust
 *
 * [674] Longest Continuous Increasing Subsequence
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut longest = 1;
        let mut current = 1;
        let mut previous_val = nums[0];
        for n in nums.iter().skip(1) {
            if *n > previous_val {
                current += 1;
            } else {
                current = 1;
            }
            longest = std::cmp::max(longest, current);
            previous_val = *n;
        }
        longest
    }
}
// @lc code=end
