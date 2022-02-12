/*
 * @lc app=leetcode id=852 lang=rust
 *
 * [852] Peak Index in a Mountain Array
 */

// @lc code=start
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut max = std::i32::MIN;
        let mut res = 0usize;
        for (i, x) in arr.into_iter().enumerate() {
            if x > max {
                max = x;
                res = i;
            } else {
                break;
            }
        }
        res as i32
    }
}
// @lc code=end

