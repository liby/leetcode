/*
 * @lc app=leetcode id=860 lang=rust
 *
 * [860] Lemonade Change
 */

// @lc code=start
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut num_5, mut num_10) = (0, 0);
        for &bill in &bills {
            if bill == 5 {
                num_5 += 1;
            } else if bill == 10 {
                match num_5 >= 1 {
                    true => num_5 -= 1,
                    false => return false,
                };
                num_10 += 1;
            } else if num_10 >= 1 && num_5 >= 1 {
                num_10 -= 1;
                num_5 -= 1;
            } else if num_5 >= 3 {
                num_5 -= 3;
            } else {
                return false;
            }
        }
        true
    }
}
// @lc code=end

