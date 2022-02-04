/*
 * @lc app=leetcode id=819 lang=rust
 *
 * [819] Most Common Word
 */

// @lc code=start
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        use std::collections::{HashMap, HashSet};

        let mut m = HashMap::new();
        let s = banned.iter().collect::<HashSet<_>>();

        let mut ans = String::new();
        let mut min_num = i32::MIN;

        for word in paragraph
            .split(&[' ', '!', '?', '\'', ',', ';', '.'][..])
            .map(|s| s.to_ascii_lowercase())
            .filter(|w| !w.is_empty() && !s.contains(w))
        {
            let num = m.entry(word.clone()).or_insert(0);
            *num += 1;
            if *num > min_num {
                min_num = *num;
                ans = word;
            }
        }
        ans
    }
}
// @lc code=end

