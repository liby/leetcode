/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seq: Vec<i32> = [0].repeat(nums.len());
        nums.into_iter().for_each(|num| seq[(num-1) as usize] += 1);
        seq.into_iter().enumerate().filter(|&(x, y)| y == 0).map(|(x, y)| (x+1) as i32).collect::<Vec<i32>>()
    }
}
// @lc code=end

