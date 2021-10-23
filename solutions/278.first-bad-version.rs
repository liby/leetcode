/*
 * @lc app=leetcode id=278 lang=rust
 *
 * [278] First Bad Version
 */

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left != right {
            let middle = left + (right - left) / 2;
            if self.isBadVersion(middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left // Guaranteed to be bad.
    }
}
// @lc code=end

