/*
 * @lc app=leetcode id=551 lang=rust
 *
 * [551] Student Attendance Record I
 */

// @lc code=start
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absCnt = 0;
        let mut lateCnt = 0;

        for char in s.chars() {
            match char {
                'L' => lateCnt += 1,
                'A' => {
                    lateCnt = 0;
                    absCnt += 1;
                }
                _ => lateCnt = 0,
            }

            if lateCnt == 3 || absCnt == 2 {
                return false;
            }
        }

        true
    }
}
// @lc code=end
