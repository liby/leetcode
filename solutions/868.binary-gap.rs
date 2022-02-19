/*
 * @lc app=leetcode id=868 lang=rust
 *
 * [868] Binary Gap
 */

// @lc code=start
impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let (mut max_dist, mut cur_dist) = (0, 0);
        n >>= n.trailing_zeros(); // drop zeros 10100000 => 101

        while n > 0 {
            if n & 1 == 1 && cur_dist > 0 {
                max_dist = max_dist.max(cur_dist);
                cur_dist = 0;
            }
            cur_dist += 1;
            n >>= 1;
        }
        max_dist
    }
}
// @lc code=end

