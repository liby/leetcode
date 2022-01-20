/*
 * @lc app=leetcode id=762 lang=rust
 *
 * [762] Prime Number of Set Bits in Binary Representation
 */

// @lc code=start
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        use std::collections::HashSet;
        let primes = [2, 3, 5, 7, 11, 13, 17, 19].iter().collect::<HashSet<_>>();
        (left..=right)
            .filter(|&x| primes.contains(&(x.count_ones())))
            .count() as _
    }
}
// @lc code=end

