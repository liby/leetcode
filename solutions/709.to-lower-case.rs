/*
 * @lc app=leetcode id=709 lang=rust
 *
 * [709] To Lower Case
 */

// @lc code=start
impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let res: String = str.chars().map(|c| {
            if c <= 'Z' && c >= 'A'{
                ((c as u8) + ('a' as u8) - ('A' as u8)) as char
            } else {
                c
            }
        }).collect();
        res
    }
}
// @lc code=end

