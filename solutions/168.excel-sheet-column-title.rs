/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 */

// @lc code=start
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut res = String::new();
        let mut n = column_number;
        while n > 0 {
            let c = (n - 1) % 26;
            res.insert(0, (b'A' + c as u8) as char);
            n = (n - 1) / 26;
        }
        res
    }
}
// @lc code=end
