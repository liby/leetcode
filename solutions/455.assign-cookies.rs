/*
 * @lc app=leetcode id=455 lang=rust
 *
 * [455] Assign Cookies
 */

// @lc code=start
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut content_children = 0;

        if g.len() == 0 || s.len() == 0 {
            return content_children;
        }

        let mut gs = g;
        let mut ss = s;
        gs.sort();
        ss.sort();
        let mut child_idx = 1;
        let mut cookie_idx = 1;

        while child_idx <= gs.len() && cookie_idx <= ss.len() {
            if gs[child_idx - 1] <= ss[cookie_idx - 1] {
                content_children += 1;
                child_idx += 1;
            }

            cookie_idx += 1;
        }

        content_children
    }
}
// @lc code=end

