/*
 * @lc app=leetcode id=405 lang=rust
 *
 * [405] Convert a Number to Hexadecimal
 */

// @lc code=start
impl Solution {
    pub fn to_hex(mut num: i32) -> String {
        const HEX: &str = "0123456789abcdef";

        if num == 0 {
            return "0".to_string();
        }

        let mut result = String::new();
        let mut count = 0;

        while num != 0 && count < 8 {
            let digit = num & 0xf;
            num >>= 4;
            result.insert(0, HEX.chars().nth(digit as usize).unwrap());
            count += 1;
        }
        result
    }
}
// @lc code=end

