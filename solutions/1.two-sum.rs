/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v = Vec::new();
        let mut d = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            let m = target - n;
            if d.contains_key(&m) {
                v = vec![d[&m] as i32, i as i32];
                break;
            } else {
                d.insert(n, i);
            }
        }
        v
    }
}
// @lc code=end
