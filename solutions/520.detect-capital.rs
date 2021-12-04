/*
 * @lc app=leetcode id=520 lang=rust
 *
 * [520] Detect Capital
 */

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() == 1 {
            return true;
        }

        let mut it = word.bytes().rev().take(word.len() - 1);
        let need_all_except_first_to_be_uppercase = it.next().unwrap().is_ascii_uppercase();

        if !it.all(|ch| ch.is_ascii_uppercase() == need_all_except_first_to_be_uppercase) {
            return false;
        }

        !need_all_except_first_to_be_uppercase /* so first can be either uppercase or lowercase */
            || word.as_bytes()[0].is_ascii_uppercase() /* need all to be uppercase */
    }
}
// @lc code=end
