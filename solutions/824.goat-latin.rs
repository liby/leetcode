/*
 * @lc app=leetcode id=824 lang=rust
 *
 * [824] Goat Latin
 */

// @lc code=start
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut v = Vec::new();
        for (n, word) in sentence.split_ascii_whitespace().enumerate() {
            let trans_word = match word.starts_with(&vowels[..]) {
                true => word.to_string(),
                false => word[1..].to_string() + &word[..1],
            } + "ma";
            v.push(trans_word + &"a".repeat(n + 1))
        }
        v.join(" ")
    }
}
// @lc code=end

