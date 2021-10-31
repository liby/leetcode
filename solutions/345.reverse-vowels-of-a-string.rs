/*
 * @lc app=leetcode id=345 lang=rust
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() == 0 {
            return s;
        }

        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        let mut s: Vec<char> = s.chars().collect();
        let mut beg = 0;
        let mut end = s.len() - 1;

        loop {
            while beg < end && !vowels.contains(&s[beg]) {
                beg += 1;
            }
            while beg < end && !vowels.contains(&s[end]) {
                end -= 1;
            }

            if beg >= end {
                break;
            }

            s.swap(beg, end);

            beg += 1;
            end -= 1;
        }

        s.into_iter().collect()
    }
}
// @lc code=end
