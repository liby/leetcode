/*
 * @lc app=leetcode id=219 lang=rust
 *
 * [219] Contains Duplicate II
 */

// @lc code=start
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        // put num as key in hashmap with most recent index of that num in nums as value
        let mut map = std::collections::HashMap::new();
        for i in (0..nums.len()) {
            match map.insert(nums[i], i) {
                //HashMap::insert returns an Option<val> of previous value for given key
                // some solutions are casting k as usize once rather than casting the indices
                // as i32, however this could overflow as there is no guarantee that k will
                // fit as usize, so even if there is some performance hit doing it this way
                // it is preferable.
                Some(x) if (i - x) as i32 <= k => return true,
                _ => continue,
            }
        }
        false
    }
}
// @lc code=end
