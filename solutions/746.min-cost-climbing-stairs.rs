/*
 * @lc app=leetcode id=746 lang=rust
 *
 * [746] Min Cost Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        cost.insert(0, 0);
        let mut cache = vec![-1; cost.len()+1];
        Self::min_cost(&cost, 0, &mut cache)
    }

    fn min_cost(cost: &[i32], index: usize, cache: &mut [i32]) -> i32 {
        if index >= cost.len() {
            0
        } else if cache[index] != -1 {
            cache[index]
        } else {
            let cc = cost[index];
            let a = Self::min_cost(cost, index+1, cache);
            let b = Self::min_cost(cost, index+2, cache);
            let ans = a.min(b) + cc;
            cache[index] = ans;
            ans
        }
    }
}
// @lc code=end

