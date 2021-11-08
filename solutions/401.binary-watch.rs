/*
 * @lc app=leetcode id=401 lang=rust
 *
 * [401] Binary Watch
 */

// @lc code=start
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        (0_u32..720_u32)
            .into_iter()
            .map(|i| (i / 60, i % 60))
            .filter(|(h, m)| (*h).count_ones() + (*m).count_ones() == turned_on as u32)
            .map(|(h, m)| format!("{}:{:02}", h, m))
            .collect::<Vec<String>>()
    }
}
// @lc code=end
