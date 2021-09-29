/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input array is sorted
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut hi, mut lo) = (numbers.len() - 1, 0);

        while lo < hi {
            let sum = numbers[lo] + numbers[hi];

            if sum == target {
                return vec![lo as i32 + 1, hi as i32 + 1];
            } else if sum < target {
                lo += 1;
            } else {
                hi -= 1;
            }
        }

        return vec![-1, -1];
    }
}
// @lc code=end
