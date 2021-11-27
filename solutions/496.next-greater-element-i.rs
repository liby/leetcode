/*
 * @lc app=leetcode id=496 lang=rust
 *
 * [496] Next Greater Element I
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![-1; nums1.len()];
        let mut nums_map: HashMap<&i32, usize> = HashMap::new();

        for (i, num) in nums2.iter().enumerate() {
            nums_map.insert(num, i);
        }

        for (i, num) in nums1.iter().enumerate() {
            for j in *nums_map.get(num).unwrap()..nums2.len() {
                if nums2[j] > *num {
                    stack[i] = nums2[j];
                    break;
                }
            }
        }
        stack
    }
}
// @lc code=end
