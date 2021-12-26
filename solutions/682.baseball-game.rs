/*
 * @lc app=leetcode id=682 lang=rust
 *
 * [682] Baseball Game
 */

// @lc code=start
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for op in ops.iter() {
            if op == "C" {
                stack.pop();
            } else if op == "D" {
                stack.push(stack[stack.len() - 1] * 2);
            } else if op == "+" {
                stack.push(stack[stack.len() - 1] + stack[stack.len() - 2]);
            } else {
                stack.push(op.parse::<i32>().unwrap());
            }
        }

        stack.iter().sum()
    }
}
// @lc code=end
