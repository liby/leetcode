/*
 * @lc app=leetcode id=628 lang=rust
 *
 * [628] Maximum Product of Three Numbers
 */

// @lc code=start
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let last = nums[(n - 3)..n].iter().fold(1, |mut sum, &val| {
            sum *= val;
            sum
        });
        let first = nums[n - 1]
            * nums[0..2].iter().fold(1, |mut sum, &val| {
                sum *= val;
                sum
            });

        std::cmp::max(first, last)
    }
}
// @lc code=end
