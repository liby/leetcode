/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */

// @lc code=start
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        loop {
            let mut sum = 0;
            while n > 0 {
                let d = n % 10;
                sum += d * d;
                n /= 10;
            }
            match sum {
                1 => return true,
                4 => return false,
                _ => n = sum,
            }
        }
    }
}
// @lc code=end
