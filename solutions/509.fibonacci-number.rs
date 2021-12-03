/*
 * @lc app=leetcode id=509 lang=rust
 *
 * [509] Fibonacci Number
 */

// @lc code=start
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut fibVec = Vec::new();
        fibVec.push(0);
        fibVec.push(1);

        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            for i in 0..n {
                fibVec.push(fibVec[i as usize] + fibVec[i as usize + 1]);
            }
        }
        fibVec[n as usize]
    }
}
// @lc code=end
