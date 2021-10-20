/*
 * @lc app=leetcode id=258 lang=rust
 *
 * [258] Add Digits
 */

// @lc code=start
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut n = num;
        while n >= 10 {
            let mut sum = 0;
            while n != 0 {
                let m = n % 10;
                sum += m;
                n /= 10;
            }
            n = sum;
        }
        n
    }
}
// @lc code=end

