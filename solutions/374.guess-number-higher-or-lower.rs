/*
 * @lc app=leetcode id=374 lang=rust
 *
 * [374] Guess Number Higher or Lower
 */

// @lc code=start
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        let mut mid: i32;
        loop {
            mid = low + (high - low) / 2;
            match guess(mid) {
                0 => break,
                -1 => high = mid - 1,
                _ => low = mid + 1,
            }
        }
        mid
    }
}
// @lc code=end
