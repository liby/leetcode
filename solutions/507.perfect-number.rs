/*
 * @lc app=leetcode id=507 lang=rust
 *
 * [507] Perfect Number
 */

// @lc code=start
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        num >= 2
            && (1..)
                .take_while(|x| x * x <= num)
                .filter(|x| num % x == 0)
                .fold(0, |acc, x| {
                    acc + {
                        if x * x == num || x == 1 {
                            x
                        } else {
                            x + num / x
                        }
                    }
                })
                == num
    }
}
// @lc code=end
