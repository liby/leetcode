/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut d: [usize; 256] = [0; 256];
        for c in magazine.chars().map(|c| (c as u8) as usize) {
            d[c] += 1;
        }
        for c in ransom_note.chars().map(|c| (c as u8) as usize) {
            if d[c] == 0 {
                return false;
            }
            d[c] -= 1;
        }
        true
    }
}
// @lc code=end
