/*
 * @lc app=leetcode id=821 lang=rust
 *
 * [821] Shortest Distance to a Character
 */

// @lc code=start
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut res = Vec::with_capacity(s.len());
        let arr_pos = s
            .char_indices()
            .filter_map(|(i, cc)| if cc == c { Some(i as i32) } else { None })
            .collect::<Vec<_>>();

        let l = arr_pos.len();
        let (mut cur_pos, mut next_pos) = (arr_pos[0], arr_pos[0]);
        let (mut idx_pos, mut mid_pos) = (0, 0);

        for i in 0..s.len() as i32 {
            if i == next_pos {
                res.push(0);

                cur_pos = arr_pos[idx_pos];
                idx_pos = (idx_pos + 1).min(l - 1);
                next_pos = arr_pos[idx_pos];
                mid_pos = (next_pos + cur_pos) / 2;
                continue;
            }

            match idx_pos > 0 && idx_pos < l {
                true if i <= mid_pos => res.push((cur_pos - i).abs()),
                true => res.push((next_pos - i).abs()),
                false => res.push((cur_pos - i).abs()),
            }
        }
        res
    }
}
// @lc code=end

