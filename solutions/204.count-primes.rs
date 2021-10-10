/*
 * @lc app=leetcode id=204 lang=rust
 *
 * [204] Count Primes
 */

// @lc code=start
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut not_prime = vec![false; n];
        let mut cnt = 0;

        for i in 2..n {
            if !not_prime[i] {
                cnt += 1;

                let mut j = 2;
                while i * j < n {
                    not_prime[i * j] = true;
                    j += 1;
                }
            }
        }

        cnt
    }
}
// @lc code=end
