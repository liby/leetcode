/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    // pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    //     match nums.binary_search(&target) {
    //         Ok(i) => i as i32,
    //         _ => {
    //             let mut i = 0;
    //             while i < nums.len() && nums[i] < target {
    //                 i += 1;
    //             }
    //             i as i32
    //         }
    //     }
    // }
    pub fn search_insert(mut nums: Vec<i32>, target: i32) -> i32 {
        if !nums.contains(&target) {
            nums.push(target);
            nums.sort();
        }

        match nums.iter().position(|&x| x == target) {
            Some(i) => i as i32,
            None => 0,
        }
    }
}
// @lc code=end
