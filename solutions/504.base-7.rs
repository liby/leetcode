/*
 * @lc app=leetcode id=504 lang=rust
 *
 * [504] Base 7
 */

// @lc code=start
impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        let mut flag = 0;
        if num == 0 {
            return 0.to_string();
        }
        if num < 0 {
            flag = 1;
            num = -num;
        }

        let mut temp = String::new();
        while num > 0 {
            let op = num % 7;
            num /= 7;
            temp = op.to_string() + &temp;
        }
        if flag == 1 {
            temp = "-".to_owned() + &temp;
        }
        temp
    }
}
// @lc code=end
