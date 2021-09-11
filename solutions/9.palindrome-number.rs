/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    fn reverse(x: i32) -> i32 {
        let mut result = 0i32;
        let mut x = x;

        while x != 0 {
            let tail = x % 10;
            x /= 10;
            if result > i32::MAX / 10 || result < i32::MIN / 10 {
                return 0;
            }
            result = result * 10 + tail;
        }
        result
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else {
            let x_copy = Solution::reverse(x);
            return x_copy == x;
        }
    }
}
// @lc code=end
