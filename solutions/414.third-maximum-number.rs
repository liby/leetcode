/*
 * @lc app=leetcode id=414 lang=rust
 *
 * [414] Third Maximum Number
 */

// @lc code=start
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut first = None;
        let mut second = None;
        let mut third = None;
        let mut tmp;

        for n in nums {
            let n = Some(n);
            if n > first {
                tmp = second;
                second = first;
                first = n;
                third = tmp;
            } else if n < first && n > second {
                tmp = second;
                second = n;
                third = tmp;
            } else if n < second && n >= third {
                third = n;
            }
        }
        return third.unwrap_or(first.unwrap());
    }
}
// @lc code=end
