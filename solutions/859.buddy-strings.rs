/*
 * @lc app=leetcode id=859 lang=rust
 *
 * [859] Buddy Strings
 */

// @lc code=start
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let mut counter = [0; 26];
        let mut swap_flag = 0;
        let mut swap_pair = (0, 0);

        for (&b1, &b2) in s.as_bytes().iter().zip(goal.as_bytes().iter()) {
            if b1 != b2 {
                match swap_flag {
                    0 => {
                        swap_pair = (b1, b2);
                        swap_flag = 1
                    }
                    1 if swap_pair.0 == b2 && swap_pair.1 == b1 => swap_flag = 2,
                    _ => return false,
                }
            }
            counter[(b1 - b'a') as usize] += 1;
        }

        swap_flag == 2 || swap_flag == 0 && counter.iter().any(|n| *n > 1)
    }
}
// @lc code=end

