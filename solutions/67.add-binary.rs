/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start
impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        // format!(
        //     "{:b}",
        //     u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
        // )
        let mut i = match a.len() > b.len() {
            true => a.len(),
            false => b.len(),
        };
        let mut result = String::with_capacity(i + 1);
        let mut carry = false;

        loop {
            let a_bin = a.pop().unwrap_or('0');
            let b_bin = b.pop().unwrap_or('0');

            if a_bin == '0' {
                if carry && b_bin == '0' {
                    // carry + a_bin + b_bin
                    // 1 + 0 + 0 = 1
                    result.insert(0, '1');
                    carry = false;
                } else if carry && b_bin == '1' {
                    // 1 + 0 + 1 = 10
                    result.insert(0, '0')
                } else {
                    // 0 + 0 + ? = ?
                    result.insert(0, b_bin);
                }
            } else {
                if carry && b_bin == '0' {
                    // 1 + 1 + 0 = 10
                    result.insert(0, '0');
                } else if carry && b_bin == '1' {
                    // 1 + 1 + 1 = 11
                    result.insert(0, '1');
                } else if !carry && b_bin == '0' {
                    // 0 + 1 + 0 = 1
                    result.insert(0, '1');
                } else if !carry && b_bin == '1' {
                    // 0 + 1 + 1 = 10
                    result.insert(0, '0');
                    carry = true;
                }
            }

            if i <= 1 {
                break;
            }

            i -= 1;
        }

        if carry {
            result.insert(0, '1');
        }
        result
    }
}
// @lc code=end
