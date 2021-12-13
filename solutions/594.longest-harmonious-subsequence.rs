/*
 * @lc app=leetcode id=594 lang=rust
 *
 * [594] Longest Harmonious Subsequence
 */

// @lc code=start
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut hm = std::collections::HashMap::new();
        nums.iter()
            .for_each(|&num| *hm.entry(num).or_insert(0) += 1);
        hm.iter().fold(0, |acc, (&num, &count)| {
            hm.get(&(num + 1)).map_or(acc, |c| acc.max(count + c))
        })
    }
}
// @lc code=end
