/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
        return false;
        }
        let mut buckets = [0; 26];
        for (i, j) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
            buckets[(i - b'a') as usize] += 1;
            buckets[(j - b'a') as usize] -= 1;
        }
        buckets == [0; 26]
    }
}
// @lc code=end

