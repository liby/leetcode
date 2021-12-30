/*
 * @lc app=leetcode id=697 lang=rust
 *
 * [697] Degree of an Array
 */

// @lc code=start
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering::*;

        let (mut degree, mut min_len) = (1, 1);
        let (mut counter, mut indicies) = ([0; 50_000], [-1; 50_000]);

        for (x, ind) in nums.iter().map(|&x| x as usize).zip(0..) {
            let (num, first) = (&mut counter[x], &mut indicies[x]);
            let cur_len = ind - *first + 1;

            *num += 1;
            if *first == -1 {
                *first = ind;
            }

            match degree.cmp(num) {
                Less => {
                    degree = *num;
                    min_len = cur_len;
                }
                Equal if cur_len < min_len => min_len = cur_len,
                _ => (),
            }
        }
        min_len
    }
}
// @lc code=end
