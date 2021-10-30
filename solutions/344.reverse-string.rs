/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        if len > 0 {
            let mut midpoint = len / 2;
            if len % 2 != 0 {
                midpoint += 1;
            }
            for i in 0..midpoint {
                let j = len - i - 1;
                let buffer = s[i];
                s[i] = s[j];
                s[j] = buffer;
            }
        }
    }
}
// @lc code=end
