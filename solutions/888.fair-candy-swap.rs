/*
 * @lc app=leetcode id=888 lang=rust
 *
 * [888] Fair Candy Swap
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sum_a = alice_sizes.iter().sum::<i32>();
        let sum_b = bob_sizes.iter().sum::<i32>();
        let set_a = alice_sizes.iter().fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });

        for y in bob_sizes.iter() {
            let x = (sum_a - sum_b) / 2 + y;
            if set_a.contains(&x) {
                return vec![x, *y];
            }
        }

        unreachable!()
    }
}
// @lc code=end

