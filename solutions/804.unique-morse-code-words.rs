/*
 * @lc app=leetcode id=804 lang=rust
 *
 * [804] Unique Morse Code Words
 */
 // @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        static MORSE: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];

        words
            .into_iter()
            .map(|w| {
                w.chars()
                    .map(|c| MORSE[c as usize - 'a' as usize])
                    .collect::<String>()
            })
            .collect::<HashSet<_>>()
            .len() as _
    }
}
// @lc code=end

