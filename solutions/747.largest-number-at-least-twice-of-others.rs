/*
 * @lc app=leetcode id=747 lang=rust
 *
 * [747] Largest Number At Least Twice of Others
 */

// @lc code=start
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut ind = -1;
        let (mut max, mut prev_max) = (0, 0);

        nums.iter().enumerate().for_each(|(i, &x)| {
            if x > max {
                prev_max = std::mem::replace(&mut max, x);
                ind = i as i32;
            } else if x > prev_max {
                prev_max = x;
            }
        });

        match max >= prev_max * 2 {
            true => ind,
            false => -1,
        }
    }
}
// @lc code=end

